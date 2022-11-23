use anyhow::{bail, Error};
use http_body_util::BodyExt;
use hyper::{
    body::{Buf, Incoming},
    Response, StatusCode, Method,
};
use tokio::net::UnixStream;

use crate::{
    app_config::AppConfig,
    container_info::ContainerInfo,
    docker_config::{DockerConfig, Endpoint},
    http_client::{build_request, build_uri, connect_tcp_stream, send_get_post},
    webhook::{notify_webhook_failure, notify_webhook_success},
};

pub struct Docker {
    pub config: DockerConfig,
}

impl Docker {
    pub async fn get_container_info(
        &self,
        app_config: &AppConfig,
    ) -> Result<Vec<ContainerInfo>, anyhow::Error> {
        let filters = build_filters(&app_config.autoheal_container_label);

        let path_and_query = format!("/containers/json?filters={filters}");

        let response = self.send_request(&path_and_query, Method::GET).await?;

        let r = response.collect().await?.aggregate().reader();

        Ok(serde_json::from_reader(r)?)
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

    async fn send_request(&self, path_and_query: &str, method: Method) -> Result<Response<Incoming>, Error> {
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

fn build_filters(autoheal_container_label: &str) -> String {
    let mut json: serde_json::Map<String, serde_json::Value> = serde_json::Map::from_iter([(
        "health".into(),
        serde_json::Value::Array(vec![serde_json::Value::String("unhealthy".into())]),
    )]);

    if "all" != autoheal_container_label {
        json.insert(
            "label".into(),
            serde_json::Value::Array(vec![serde_json::Value::String(format!(
                "{}=true",
                autoheal_container_label
            ))]),
        );
    };

    percent_encoding::percent_encode(
        serde_json::Value::Object(json).to_string().as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use percent_encoding::percent_decode;
    use serde_json::json;

    use crate::docker::build_filters;

    #[test]
    fn test_build_filters_all() {
        let all_unhealthy = build_filters("all");

        assert_eq!(all_unhealthy, "%7B%22health%22%3A%5B%22unhealthy%22%5D%7D");

        let decoded = percent_decode(all_unhealthy.as_bytes());

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&decoded.decode_utf8().unwrap()).unwrap(),
            json!({ "health": ["unhealthy"] })
        );
    }

    #[test]
    fn test_build_filters_autoheal() {
        let autoheal_and_unhealthy = build_filters("autoheal");
        assert_eq!(autoheal_and_unhealthy, "%7B%22health%22%3A%5B%22unhealthy%22%5D%2C%22label%22%3A%5B%22autoheal%3Dtrue%22%5D%7D");

        let decoded = percent_decode(autoheal_and_unhealthy.as_bytes());

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&decoded.decode_utf8().unwrap()).unwrap(),
            json!({ "health": ["unhealthy"], "label": ["autoheal=true"] })
        );
    }

    #[test]
    fn test_build_filters_custom() {
        let custom_and_unhealthy = build_filters("custom");
        assert_eq!(
            custom_and_unhealthy,
            "%7B%22health%22%3A%5B%22unhealthy%22%5D%2C%22label%22%3A%5B%22custom%3Dtrue%22%5D%7D"
        );

        let decoded = percent_decode(custom_and_unhealthy.as_bytes());

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&decoded.decode_utf8().unwrap()).unwrap(),
            json!({ "health": ["unhealthy"], "label": ["custom=true"] })
        );
    }
}
