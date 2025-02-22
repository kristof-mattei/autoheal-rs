use std::rc::Rc;
use std::time::Duration;

use color_eyre::eyre;
use color_eyre::eyre::bail;
use http::Uri;
use http_body_util::BodyExt;
use hyper::body::{Buf, Incoming};
use hyper::{Method, Response, StatusCode};
use hyper_tls::HttpsConnector;
use hyper_unix_socket::UnixSocketConnector;
use tokio::time::timeout;
use tracing::{Level, event};

use crate::app_config::AppConfig;
use crate::container::Container;
use crate::docker_config::{DockerConfig, Endpoint};
use crate::http_client::{build_request, execute_request};
use crate::webhook::{notify_webhook_failure, notify_webhook_success};

pub struct Docker {
    config: DockerConfig,
    encoded_filters: Rc<str>,
}

impl Docker {
    pub fn new(config: DockerConfig, filters: &serde_json::Value) -> Self {
        let encoded_filters = crate::encoding::url_encode(filters);

        Self {
            config,
            encoded_filters: Rc::from(encoded_filters),
        }
    }

    pub async fn get_containers(&self) -> Result<Vec<Container>, eyre::Report> {
        let path_and_query = format!("/containers/json?filters={}", self.encoded_filters);

        let response = self.send_request(&path_and_query, Method::GET).await?;

        let reader = response.collect().await?.aggregate().reader();

        let result = serde_json::from_reader::<_, Vec<Container>>(reader)?;

        Ok(result)
    }

    pub async fn restart_container(
        &self,
        container_id: &str,
        timeout: u32,
    ) -> Result<(), eyre::Report> {
        let path_and_query = format!("/containers/{}/restart?t={}", container_id, timeout);

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
    ) -> Result<Response<Incoming>, eyre::Report> {
        match &self.config.endpoint {
            Endpoint::Direct {
                url,
                timeout_milliseconds,
            } => {
                let connector = HttpsConnector::new();
                let request = build_request(url.clone(), path_and_query, method)?;

                let response = execute_request(connector, request);

                match timeout(Duration::from_millis(*timeout_milliseconds), response).await {
                    Ok(Ok(o)) => Ok(o),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(e.into()),
                }
            },
            Endpoint::Socket(socket) => {
                let connector = UnixSocketConnector::new(socket.clone());

                let request =
                    build_request(Uri::from_static("http://localhost"), path_and_query, method)?;

                execute_request(connector, request).await
            },
        }
    }

    pub async fn check_container_health(
        &self,
        app_config: &AppConfig,
        container_info: &Container,
        times: usize,
    ) {
        let container_short_id = &container_info.id[0..12];

        match container_info.get_name() {
            None => {
                event!(
                    Level::ERROR,
                    "Container name of {} is null, which implies container does not exist - don't restart.",
                    container_short_id
                );
            },
            Some(container_names) => {
                if &*container_info.state == "restarting" {
                    event!(
                        Level::INFO,
                        "Container {} ({}) found to be restarting - don't restart.",
                        container_names,
                        container_short_id
                    );
                } else {
                    let timeout = container_info
                        .timeout
                        .unwrap_or(app_config.autoheal_default_stop_timeout);

                    event!(
                        Level::INFO,
                        "Container {} ({}) found to be unhealthy {} times. Restarting container now with {}s timeout.",
                        container_names,
                        container_short_id,
                        times,
                        timeout
                    );

                    match self.restart_container(container_short_id, timeout).await {
                        Ok(()) => {
                            notify_webhook_success(app_config, container_short_id, container_names);
                        },
                        Err(e) => {
                            event!(Level::INFO,
                                error = ?e,
                                "Restarting container {} ({}) failed.",
                                container_names,
                                container_short_id
                            );

                            notify_webhook_failure(
                                app_config,
                                container_names,
                                container_short_id,
                                e,
                            );
                        },
                    }
                }
            },
        }
    }
}
