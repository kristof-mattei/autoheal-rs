use std::convert::Infallible;
use std::env;
use std::env::VarError;
use std::rc::Rc;
use std::time::Duration;

use app_config::RawConfig;
use color_eyre::config::HookBuilder;
use color_eyre::eyre;
use docker_connection::DockerConnection;
use docker_healer::DockerHealer;
use ffi_handlers::set_up_handlers;
use hashbrown::HashMap;
use tokio::time::sleep;
use tracing::{Level, event};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::{EnvFilter, Layer as _};

mod app_config;
mod container;
mod docker_connection;
mod docker_healer;
mod encoding;
mod ffi_handlers;
mod helpers;
mod http_client;
mod unhealthy_filters;
mod webhook;

fn build_default_filter() -> EnvFilter {
    EnvFilter::builder()
        .parse(format!("INFO,{}=TRACE", env!("CARGO_CRATE_NAME")))
        .expect("Default filter should always work")
}

fn init_tracing() -> Result<(), eyre::Report> {
    let (filter, filter_parsing_error) = match env::var(EnvFilter::DEFAULT_ENV) {
        Ok(user_directive) => match EnvFilter::builder().parse(user_directive) {
            Ok(filter) => (filter, None),
            Err(error) => (build_default_filter(), Some(eyre::Report::new(error))),
        },
        Err(VarError::NotPresent) => (build_default_filter(), None),
        Err(error @ VarError::NotUnicode(_)) => {
            (build_default_filter(), Some(eyre::Report::new(error)))
        },
    };

    let registry = tracing_subscriber::registry();

    #[cfg(feature = "tokio-console")]
    let registry = registry.with(console_subscriber::ConsoleLayer::builder().spawn());

    registry
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .with(tracing_error::ErrorLayer::default())
        .try_init()?;

    filter_parsing_error.map_or(Ok(()), Err)
}

fn main() -> Result<(), eyre::Report> {
    HookBuilder::default()
        .capture_span_trace_by_default(true)
        .display_env_section(false)
        .install()?;

    init_tracing()?;

    set_up_handlers()?;

    // initialize the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // start service
    rt.block_on(healer())?;

    Ok(())
}

async fn healer() -> Result<Infallible, eyre::Report> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    event!(Level::INFO, "{} v{}", name, version);

    let (docker_startup_config, runtime_config, webhook_url) = RawConfig::build()?;

    // let timeout_milliseconds = try_parse_env_variable_with_default("CURL_TIMEOUT", 30000)?;

    let docker = DockerHealer::new(
        DockerConnection::build(docker_startup_config)?,
        &unhealthy_filters::build(runtime_config.container_label.as_deref()),
        webhook_url,
    )?;

    // TODO define failure mode
    // Do we fail? Do we retry?

    if runtime_config.start_period > 0 {
        event!(
            Level::INFO,
            "Monitoring containers for unhealthy status in {} second(s)",
            runtime_config.start_period
        );

        sleep(Duration::from_secs(runtime_config.start_period)).await;
    }

    let mut history_unhealthy = HashMap::<Rc<str>, (Option<Rc<str>>, usize)>::new();

    #[expect(clippy::infinite_loop, reason = "Endless task")]
    loop {
        match docker.get_containers().await {
            Ok(containers) => {
                let mut current_unhealthy: HashMap<Rc<str>, Option<Rc<str>>> = containers
                    .iter()
                    .map(|c| (Rc::clone(&c.id), c.get_name().map(Into::into)))
                    .collect::<HashMap<_, _>>();

                for container in containers {
                    if container
                        .names
                        .iter()
                        .any(|n| runtime_config.exclude_containers.contains(n))
                    {
                        event!(
                            Level::INFO,
                            "Container {} ({}) is unhealthy, but it is excluded",
                            container
                                .get_name()
                                .as_deref()
                                .unwrap_or("<UNNAMED CONTAINER>"),
                            &container.id[0..12],
                        );

                        continue;
                    }

                    docker
                        .check_container_health(
                            &runtime_config,
                            &container,
                            history_unhealthy
                                .get(&container.id)
                                .map_or(1, |&(_, t)| t + 1),
                        )
                        .await;
                }

                history_unhealthy = history_unhealthy
                    .into_iter()
                    .filter_map(|(key, (names, times))| {
                        if let Some(new_name) = current_unhealthy.remove(&key) {
                            // still unhealthy
                            // take the new name
                            Some((key, (new_name, times + 1)))
                        } else {
                            // healthy
                            event!(
                                Level::INFO,
                                "Container {} ({}) returned to healthy state.",
                                names.as_deref().unwrap_or("<UNNAMED CONTAINER>"),
                                key
                            );
                            None
                        }
                    })
                    .collect();
            },
            Err(err) => {
                event!(Level::ERROR, ?err, "Failed to fetch container info");
            },
        }

        sleep(Duration::from_secs(runtime_config.interval)).await;
    }
}

#[cfg(test)]
mod tests {}
