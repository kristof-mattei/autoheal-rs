#![cfg_attr(not(debug_assertions), deny(warnings))]

use app_config::AppConfig;
use container_info::ContainerInfo;
use docker::Docker;
use docker_config::DockerConfig;
use handlers::set_up_handlers;
use std::{convert::Infallible, time::Duration};
use tokio::time::sleep;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
mod app_config;
mod container_info;
mod docker;
mod docker_config;
mod env;
mod handlers;
mod helpers;
mod http_client;
mod utils;
mod webhook;

use crate::webhook::{notify_webhook_failure, notify_webhook_success};

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
        println!(
            "Monitoring containers for unhealthy status in {} second(s)",
            app_config.autoheal_start_period
        );
        sleep(Duration::from_secs(app_config.autoheal_start_period)).await;
    }

    loop {
        match docker.get_container_info(&app_config).await {
            Ok(container_infos) => {
                for c_i in container_infos {
                    check_container_health(&docker, &app_config, c_i).await;
                }
            },
            Err(_) => todo!(),
        }

        sleep(Duration::from_secs(app_config.autoheal_interval)).await;
    }
}

async fn check_container_health(
    docker: &Docker,
    app_config: &AppConfig,
    container_info: ContainerInfo,
) {
    let container_short_id = &container_info.id[0..12];
    let date = chrono::offset::Utc::now();

    match &container_info.name {
        None => {
            eprintln!("{date} Container name of {container_short_id} is null, which implies container does not exist - don't restart");
        },
        Some(container_name) => {
            if container_info.state == "restarting" {
                println!("{date} Container {container_name:?} ({container_short_id}) found to be restarting - don't restart");
            } else {
                let timeout = container_info
                    .timeout
                    .unwrap_or(app_config.autoheal_default_stop_timeout);

                println!("{date} Container {container_name} ({container_short_id}) found to be unhealthy - Restarting container now with {}s timeout", timeout);

                match docker.restart_container(&container_info.id, timeout).await {
                    Ok(()) => {
                        notify_webhook_success(app_config, container_short_id, container_name);
                    },
                    Err(e) => {
                        eprintln!("{date} Restarting container ({container_short_id}) failed. Error: {e:?}");
                        notify_webhook_failure(app_config, container_name, container_short_id, &e);
                    },
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {}
