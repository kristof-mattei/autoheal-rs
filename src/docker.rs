use anyhow::{bail, Error};
use http_body_util::BodyExt;
use hyper::{
    body::{Buf, Incoming},
    Response, StatusCode,
};
use serde_json::json;
use tokio::net::UnixStream;

use crate::{
    app_config::AppConfig,
    container_info::ContainerInfo,
    docker_config::{DockerConfig, Endpoint},
    http_client::{build_request, build_uri, connect_tcp_stream, send_get_post},
};

pub struct Docker<'a> {
    pub docker_config: DockerConfig,
    pub app_config: &'a AppConfig,
}

impl<'a> Docker<'a> {
    pub async fn get_container_info(&self) -> Result<Vec<ContainerInfo>, anyhow::Error> {
        let mut json = serde_json::Map::new();
        json.insert(
            "health".into(),
            serde_json::Value::Array(vec![serde_json::Value::String("unhealthy".into())]),
        );

        if "all" != self.app_config.autoheal_container_label {
            json.insert(
                "label".into(),
                serde_json::Value::Array(vec![serde_json::Value::String(format!(
                    "{}=true",
                    self.app_config.autoheal_container_label
                ))]),
            );
        };

        let encoded = percent_encoding::percent_encode(
            serde_json::Value::Object(json).to_string().as_bytes(),
            percent_encoding::NON_ALPHANUMERIC,
        )
        .to_string();

        let path_and_query = format!("/containers/json?filters={}", encoded);

        let response = self.send_request(&path_and_query).await?;

        let r = response.collect().await?.aggregate().reader();

        Ok(serde_json::from_reader(r)?)
    }

    pub async fn restart_container(
        &self,
        container_id: &str,
        timeout: u32,
    ) -> Result<(), anyhow::Error> {
        let path_and_query = format!("/containers/{container_id}/restart?t={timeout}");

        let response = self.send_request(&path_and_query).await?;

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

    async fn send_request(&self, path_and_query: &str) -> Result<Response<Incoming>, Error> {
        match &self.docker_config.endpoint {
            Endpoint::Direct(url) => {
                let stream = connect_tcp_stream(url).await?;
                let request = build_request(&build_uri(url.clone(), path_and_query)?)?;
                send_get_post(stream, request).await
            },
            Endpoint::Socket(socket, url) => {
                let stream = UnixStream::connect(&socket).await?;
                let request = build_request(&build_uri(url.clone(), path_and_query)?)?;
                send_get_post(stream, request).await
            },
        }
    }
}

#[cfg(test)]
mod tests {}
