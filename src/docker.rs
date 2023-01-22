use anyhow::{bail, Error};
use http_body_util::BodyExt;
use hyper::body::{Buf, Incoming};
use hyper::{Method, Response, StatusCode};
use tokio::net::UnixStream;

use crate::app_config::AppConfig;
use crate::container_info::ContainerInfo;
use crate::docker_config::{DockerConfig, Endpoint};
use crate::http_client::{build_request, build_uri, connect_tcp_stream, send_get_post};
use crate::webhook::{notify_webhook_failure, notify_webhook_success};

pub struct Docker {
    config: DockerConfig,
    encoded_filters: String,
}

impl Docker {
    pub fn new(config: DockerConfig, filters: &serde_json::Value) -> Self {
        let encoded_filters = crate::encoding::url_encode(filters);

        Self {
            config,
            encoded_filters,
        }
    }

    pub async fn get_container_info(&self) -> Result<Vec<ContainerInfo>, anyhow::Error> {
        let path_and_query = format!("/containers/json?filters={}", self.encoded_filters);

        let response = self.send_request(&path_and_query, Method::GET).await?;

        let reader = response.collect().await?.aggregate().reader();

        let result = serde_json::from_reader::<_, Vec<ContainerInfo>>(reader)?;

        Ok(result)
    }

    pub async fn restart_container(
        &self,
        container_id: &str,
        timeout: u32,
    ) -> Result<(), anyhow::Error> {
        let path_and_query = format!("/containers/{container_id}/restart?t={timeout}");

        let response = self.send_request(&path_and_query, Method::POST).await?;

        let status_code = response.status();

        if StatusCode::is_success(&status_code) {
            Ok(())
        } else {
            bail!(format!(
                "Tried to refresh container but it failed with {:?}",
                status_code
            ));
        }
    }

    async fn send_request(
        &self,
        path_and_query: &str,
        method: Method,
    ) -> Result<Response<Incoming>, Error> {
        match &self.config.endpoint {
            Endpoint::Direct(url) => {
                let stream = connect_tcp_stream(url).await?;
                let request = build_request(&build_uri(url.clone(), path_and_query)?, method)?;
                send_get_post(stream, request).await
            },
            Endpoint::Socket(socket, url) => {
                let stream = UnixStream::connect(&socket).await?;
                let request = build_request(&build_uri(url.clone(), path_and_query)?, method)?;
                send_get_post(stream, request).await
            },
        }
    }

    pub async fn check_container_health(
        &self,
        app_config: &AppConfig,
        container_info: ContainerInfo,
    ) {
        let container_short_id = &container_info.id[0..12];

        match &container_info.name {
            None => {
                tracing::error!("Container name of {} is null, which implies container does not exist - don't restart.", container_short_id);
            },
            Some(container_name) => {
                if container_info.state == "restarting" {
                    tracing::info!(
                        "Container {} ({}) found to be restarting - don't restart.",
                        container_name,
                        container_short_id
                    );
                } else {
                    let timeout = container_info
                        .timeout
                        .unwrap_or(app_config.autoheal_default_stop_timeout);

                    tracing::info!(
                        "Container {} ({}) found to be unhealthy - Restarting container now with {}s timeout.",
                        container_name,
                        container_short_id, timeout
                    );

                    match self.restart_container(container_short_id, timeout).await {
                        Ok(()) => {
                            notify_webhook_success(app_config, container_short_id, container_name);
                        },
                        Err(e) => {
                            tracing::info!(
                                error = ?e,
                                "Restarting container {} ({}) failed.",
                                container_name,
                                container_short_id
                            );

                            notify_webhook_failure(
                                app_config,
                                container_name,
                                container_short_id,
                                &e,
                            );
                        },
                    }
                }
            },
        }
    }
}
