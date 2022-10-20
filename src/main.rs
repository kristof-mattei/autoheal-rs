#![cfg_attr(not(debug_assertions), deny(warnings))]

use container_info::ContainerInfo;
use docker::Docker;
use docker_config::build as build_docker_config;
use handlers::set_up_handlers;
use std::time::Duration;
use tokio::time::sleep;
mod app_config;
mod container_info;
mod docker;
mod docker_config;
mod env;
mod handlers;
mod http_client;
mod utils;
mod webhook;

use app_config::{build as build_app_config, AppConfig};

use crate::webhook::{notify_webhook_failure, notify_webhook_success};

fn main() -> Result<(), anyhow::Error> {
    // set up logger
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    set_up_handlers()?;

    // initialize the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // start service
    rt.block_on(actual_main())
}

async fn actual_main() -> Result<(), anyhow::Error> {
    let app_config = build_app_config()?;
    let docker = Docker {
        docker_config: build_docker_config()?,
    };

    // TODO, remove
    let command = "autoheal";

    // TODO check if docker socket exists

    // TODO define failure mode
    // Do we fail? Do we retry?

    if "autoheal" == command {
        if app_config.autoheal_start_period > 0 {
            println!(
                "Monitoring containers for unhealthy status in {} second(s)",
                app_config.autoheal_start_period
            );
            sleep(Duration::from_secs(app_config.autoheal_start_period)).await;
        }

        loop {
            match docker.get_container_info().await {
                Ok(container_infos) => {
                    for c_i in container_infos {
                        check_container_health(&docker, c_i, &app_config).await;
                    }
                },
                Err(_) => todo!(),
            }

            sleep(Duration::from_secs(app_config.autoheal_interval)).await;
        }
    }

    Ok(())
}

async fn check_container_health(
    docker: &Docker,
    container_info: ContainerInfo,
    app_config: &AppConfig,
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
