use std::convert::Infallible;
use std::rc::Rc;
use std::time::Duration;

use app_config::AppConfig;
use color_eyre::eyre;
use docker::Docker;
use docker_config::DockerConfig;
use handlers::set_up_handlers;
use hashbrown::HashMap;
use tokio::time::sleep;
use tracing::{Level, event};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

mod app_config;
mod container;
mod docker;
mod docker_config;
mod encoding;
mod env;
mod handlers;
mod helpers;
mod http_client;
mod unhealthy_filters;
mod webhook;

fn init_tracing(console_subscriber: bool) -> Result<(), eyre::Report> {
    let main_filter = EnvFilter::builder()
        .parse(std::env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| {
            format!("INFO,{}=TRACE", env!("CARGO_PKG_NAME").replace('-', "_"))
        }))?;

    let mut layers = vec![];

    if console_subscriber {
        layers.push(
            console_subscriber::ConsoleLayer::builder()
                .with_default_env()
                .spawn()
                .boxed(),
        );
    }

    layers.push(
        tracing_subscriber::fmt::layer()
            .with_filter(main_filter)
            .boxed(),
    );
    layers.push(tracing_error::ErrorLayer::default().boxed());

    Ok(tracing_subscriber::registry().with(layers).try_init()?)
}

fn main() -> Result<Infallible, eyre::Report> {
    color_eyre::config::HookBuilder::default()
        .capture_span_trace_by_default(false)
        .install()?;

    // TODO this param should come from env / config,
    init_tracing(true)?;

    set_up_handlers()?;

    // initialize the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // start service
    rt.block_on(healer())
}

async fn healer() -> Result<Infallible, eyre::Report> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    event!(Level::INFO, "{} v{}", name, version);

    let app_config = AppConfig::build()?;

    let docker = Docker::new(
        DockerConfig::build()?,
        &unhealthy_filters::build(app_config.autoheal_container_label.as_deref()),
    );

    // TODO define failure mode
    // Do we fail? Do we retry?

    if app_config.autoheal_start_period > 0 {
        event!(
            Level::INFO,
            "Monitoring containers for unhealthy status in {} second(s)",
            app_config.autoheal_start_period
        );
        sleep(Duration::from_secs(app_config.autoheal_start_period)).await;
    }

    let mut history_unhealthy = HashMap::<Rc<str>, (Option<Rc<str>>, usize)>::new();

    loop {
        match docker.get_containers().await {
            Ok(containers) => {
                let mut current_unhealthy: HashMap<Rc<str>, Option<Rc<str>>> = containers
                    .iter()
                    .map(|c| (c.id.clone(), c.get_name().map(Into::into)))
                    .collect::<HashMap<_, _>>();

                for container in containers {
                    if container
                        .names
                        .iter()
                        .any(|n| app_config.autoheal_exclude_containers.contains(n))
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
                            &app_config,
                            &container,
                            history_unhealthy
                                .get(&container.id)
                                .map_or(1, |(_, t)| *t + 1),
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

        sleep(Duration::from_secs(app_config.autoheal_interval)).await;
    }
}

#[cfg(test)]
mod tests {}
