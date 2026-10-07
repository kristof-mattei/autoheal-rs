mod build_env;
mod config;
mod docker_healer;
mod helpers;
mod shutdown;
mod signal_handlers;
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
use futures_util::future::{BoxFuture, FutureExt as _};
use futures_util::stream::{FuturesUnordered, StreamExt as _};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
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

    let (filter, parsing_error) = match env::var(EnvFilter::DEFAULT_ENV).as_deref().map(str::trim) {
        Ok("") | Err(&VarError::NotPresent) => (build_default_filter(), None),
        Ok(user_directive) => match EnvFilter::builder().parse(user_directive) {
            Ok(filter) => (filter, None),
            Err(error) => (build_default_filter(), Some(eyre::Report::new(error))),
        },
        Err(error @ &VarError::NotUnicode(_)) => (
            build_default_filter(),
            Some(eyre::Report::new(error.clone())),
        ),
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
        docker_config.client_credentials,
        docker_config.timeout,
    ) {
        Ok(client) => client,
        Err(error) => return Shutdown::from(error),
    };

    let docker_healer = DockerHealer::new(docker_client, healer_config, filters, webhook_url);

    let cancellation_token = CancellationToken::new();

    let mut tasks = FuturesUnordered::new();

    tasks.push(spawn_task("Monitor", {
        let cancellation_token = cancellation_token.clone();

        async move {
            cancellation_token
                .run_until_cancelled(docker_healer.monitor_containers())
                .await;

            Ok(())
        }
    }));

    // biased so that when multiple are ready at once, task failure wins over signals
    let shutdown_reason = tokio::select! {
        biased;
        Some((name, result)) = tasks.next() => {
            task_stopped(name, result)
        },
        result = signal_handlers::wait_for_sigterm() => {
            result
        },
        result = signal_handlers::wait_for_sigint() => {
            result
        },
    };

    cancellation_token.cancel();

    let drained = timeout(Duration::from_secs(10), async {
        while let Some((name, result)) = tasks.next().await {
            if let Err(report) = result {
                event!(
                    Level::ERROR,
                    task = name,
                    ?report,
                    "Task failed during the shutdown"
                );
            }
        }
    })
    .await
    .is_ok();

    if !drained {
        event!(Level::ERROR, "Task didn't stop within allotted time!");
    }

    // a shutdown that already reports a failure is returned unchanged
    if !drained && matches!(shutdown_reason, Shutdown::Success | Shutdown::Signal(_)) {
        return Shutdown::OperationalFailure {
            code: ExitCode::FAILURE,
            message: "Tasks didn't stop within the allotted time",
        };
    }

    shutdown_reason
}

type TaskResult = Result<(), eyre::Report>;

fn spawn_task<F>(name: &'static str, task: F) -> BoxFuture<'static, (&'static str, TaskResult)>
where
    F: Future<Output = TaskResult> + Send + 'static,
{
    let handle = spawn_with_name(name, task);

    async move {
        let result = match handle.await {
            Ok(result) => result,
            Err(join_error) => Err(eyre::Report::new(join_error)),
        };

        (name, result)
    }
    .boxed()
}

/// Every task runs until the shutdown, so one that stops before it is a failure.
fn task_stopped(name: &'static str, result: TaskResult) -> Shutdown {
    match result {
        Ok(()) => {
            Shutdown::UnexpectedError(eyre::eyre!("Task `{}` stopped before the shutdown", name))
        },
        Err(report) => {
            Shutdown::UnexpectedError(report.wrap_err(format!("Task `{}` failed", name)))
        },
    }
}
