mod build_env;
mod config;
mod docker_healer;
mod helpers;
mod shutdown;
mod signal_handlers;
mod task_tracker_ext;
mod unhealthy_filters;
mod utils;
mod webhook;

use std::convert::Infallible;
use std::env;
use std::env::VarError;
use std::process::{ExitCode, Termination as _};
use std::time::Duration;

use color_eyre::config::HookBuilder;
use color_eyre::eyre;
use config::AppConfig;
use docker_healer::DockerHealer;
use task_tracker_ext::TaskTrackerExt as _;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{Level, event};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::{EnvFilter, Layer as _};
use twistlock::client::Client;

use crate::build_env::get_build_env;
use crate::shutdown::Shutdown;
use crate::utils::flatten_shutdown_handle;
use crate::utils::task::spawn_with_name;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn build_filter() -> (EnvFilter, Option<eyre::Report>) {
    fn build_default_filter() -> EnvFilter {
        EnvFilter::builder()
            .parse(format!("INFO,{}=TRACE", env!("CARGO_CRATE_NAME")))
            .expect("Default filter should always work")
    }

    let (filter, parsing_error) = match env::var(EnvFilter::DEFAULT_ENV) {
        Ok(user_directive) => match EnvFilter::builder().parse(user_directive) {
            Ok(filter) => (filter, None),
            Err(error) => (build_default_filter(), Some(eyre::Report::new(error))),
        },
        Err(VarError::NotPresent) => (build_default_filter(), None),
        Err(error @ VarError::NotUnicode(_)) => {
            (build_default_filter(), Some(eyre::Report::new(error)))
        },
    };

    (filter, parsing_error)
}

fn init_tracing(filter: EnvFilter) -> Result<(), eyre::Report> {
    let registry = tracing_subscriber::registry();

    #[cfg(feature = "tokio-console")]
    let registry = registry.with(console_subscriber::ConsoleLayer::builder().spawn());

    Ok(registry
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .with(tracing_error::ErrorLayer::default())
        .try_init()?)
}

fn main() -> ExitCode {
    HookBuilder::default()
        .capture_span_trace_by_default(true)
        .display_env_section(false)
        .install()
        .expect("Failed to install panic handler");

    let (env_filter, parsing_error) = build_filter();

    init_tracing(env_filter).expect("Failed to set up tracing");

    // bubble up the parsing error
    if let Err(error) = parsing_error.map_or(Ok(()), Err) {
        return Err::<Infallible, _>(error).report();
    }

    // initialize the runtime
    let shutdown: Shutdown = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("Failed building the Runtime")
        .block_on(async {
            // explicitly launch everything in a spawned task
            // see https://docs.rs/tokio/latest/tokio/attr.main.html#non-worker-async-function
            let handle = spawn_with_name("main task runner", start_tasks());

            flatten_shutdown_handle(handle).await
        });

    shutdown.report()
}

fn print_header() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let build_env = get_build_env();

    event!(
        Level::INFO,
        "{} v{} - built for {} ({})",
        NAME,
        VERSION,
        build_env.get_target(),
        build_env.get_target_cpu().unwrap_or("base cpu variant"),
    );
}

async fn start_tasks() -> Shutdown {
    print_header();

    let AppConfig {
        docker_config,
        healer_config,
        container_label,
        webhook_url,
    } = match AppConfig::build() {
        Ok(config) => config,
        Err(error) => return Shutdown::from(error),
    };

    let filters = unhealthy_filters::build(container_label.as_deref());

    let docker_client = match Client::build(
        docker_config.docker_host,
        docker_config.cacert,
        docker_config.client_cert,
        docker_config.client_key,
        docker_config.timeout,
    ) {
        Ok(client) => client,
        Err(error) => return Shutdown::from(error),
    };

    let docker_healer = DockerHealer::new(docker_client, healer_config, filters, webhook_url);

    let cancellation_token = CancellationToken::new();

    let tasks = TaskTracker::new();

    {
        let cancellation_token = cancellation_token.clone();

        tasks.spawn_with_name("Monitor", async move {
            let _guard = cancellation_token.clone().drop_guard();

            cancellation_token
                .run_until_cancelled(docker_healer.monitor_containers())
                .await;
        });
    }

    // now we wait forever for either
    // * SIGTERM
    // * CTRL+c (SIGINT)
    // * cancellation of the shutdown token, triggered by another task when it
    //   completes unexpectedly (which means it failed)
    let shutdown_reason = tokio::select! {
        biased;
        () = cancellation_token.cancelled() => {
            event!(Level::WARN, "Underlying task stopped, stopping all other tasks");

            Shutdown::OperationalFailure {
                code: ExitCode::FAILURE,
                message: "Some task unexpectedly failed which triggered a shutdown."
            }
        },
        result = signal_handlers::wait_for_sigterm() => {
            result
        },
        result = signal_handlers::wait_for_sigint() => {
            result
        },
    };

    // catch all cancel in case we got here via something else than a cancellation token
    cancellation_token.cancel();

    tasks.close();

    // wait for the tasks that holds the server to exit gracefully
    // this is easier to write than x separate timeoouts
    // while we don't know if any of them gets killed
    // this will do for now, and we can always trace back the logs
    if timeout(Duration::from_secs(10), tasks.wait())
        .await
        .is_err()
    {
        event!(Level::ERROR, "Task didn't stop within allotted time!");
    }

    shutdown_reason
}
