#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![forbid(non_ascii_idents)]
#![allow(clippy::uninlined_format_args)]

use std::convert::Infallible;
use std::time::Duration;

use app_config::AppConfig;
use docker::Docker;
use docker_config::DockerConfig;
use handlers::set_up_handlers;
use tokio::time::sleep;
use tracing::metadata::LevelFilter;
use tracing::{info, Level};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod app_config;
mod container_info;
mod docker;
mod docker_config;
mod encoding;
mod env;
mod filters;
mod handlers;
mod helpers;
mod http_client;
mod support;
mod webhook;

fn main() -> Result<Infallible, color_eyre::Report> {
    color_eyre::install()?;

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish()
        .init();

    set_up_handlers()?;

    // initialize the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // start service
    rt.block_on(healer())
}

async fn healer() -> Result<Infallible, color_eyre::Report> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    info!("{} v{}", name, version);

    let app_config = AppConfig::build()?;

    let docker = Docker::new(
        DockerConfig::build()?,
        &filters::build(&app_config.autoheal_container_label),
    );

    // TODO define failure mode
    // Do we fail? Do we retry?

    if app_config.autoheal_start_period > 0 {
        info!(
            "Monitoring containers for unhealthy status in {} second(s)",
            app_config.autoheal_start_period
        );
        sleep(Duration::from_secs(app_config.autoheal_start_period)).await;
    }

    loop {
        match docker.get_container_info().await {
            Ok(container_infos) => {
                for c_i in container_infos {
                    docker.check_container_health(&app_config, c_i).await;
                }
            },
            Err(e) => {
                return Err(wrap_and_report!(
                    Level::ERROR,
                    e,
                    "Failed to fetch container info"
                ));
            },
        }

        sleep(Duration::from_secs(app_config.autoheal_interval)).await;
    }
}

#[cfg(test)]
mod tests {}
