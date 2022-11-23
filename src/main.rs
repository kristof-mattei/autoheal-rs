#![cfg_attr(not(debug_assertions), deny(warnings))]

use std::{convert::Infallible, time::Duration};

use app_config::AppConfig;
use docker::Docker;
use docker_config::DockerConfig;
use handlers::set_up_handlers;
use tokio::time::sleep;
use tracing::{info, metadata::LevelFilter, Level};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
mod app_config;
mod container_info;
mod docker;
mod docker_config;
mod env;
mod handlers;
mod helpers;
mod http_client;
mod webhook;

fn main() -> Result<Infallible, anyhow::Error> {
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

async fn healer() -> Result<Infallible, anyhow::Error> {
    let app_config = AppConfig::build()?;
    let docker = Docker {
        config: DockerConfig::build()?,
    };

    // TODO define failure mode
    // Do we fail? Do we retry?

    if app_config.autoheal_start_period > 0 {
        info!(
            message = "Monitoring containers for unhealthy status in {} second(s)",
            app_config.autoheal_start_period
        );
        sleep(Duration::from_secs(app_config.autoheal_start_period)).await;
    }

    loop {
        match docker.get_container_info(&app_config).await {
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
