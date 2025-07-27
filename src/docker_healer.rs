use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use color_eyre::eyre;
use http::Uri;
use http_body_util::{BodyExt as _, Full};
use hyper::body::{Buf as _, Bytes, Incoming};
use hyper::{Method, Response, StatusCode};
use hyper_rustls::{FixedServerNameResolver, HttpsConnector, HttpsConnectorBuilder};
use hyper_unix_socket::UnixSocketConnector;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use rustls::client::ClientConfig;
use rustls::{DEFAULT_VERSIONS, RootCertStore};
use tokio::time::timeout;
use tracing::{Level, event};

use crate::container::Container;
use crate::docker_connection::{DockerConnection, DockerEndpointConfig};
use crate::encoding::url_encode;
use crate::http_client::{build_client, build_request, execute_request};
use crate::{app_config::RuntimeConfig, webhook::WebHookNotifier};

enum DockerEndpoint {
    Socket(Client<UnixSocketConnector<PathBuf>, Full<Bytes>>),
    Tls {
        client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
    },
}

pub struct DockerHealer {
    endpoint: DockerEndpoint,
    encoded_filters: Rc<str>,
    timeout_milliseconds: u64,
    uri: http::Uri,
    notifier: WebHookNotifier,
}

fn build_docker_client(endpoint: DockerEndpointConfig) -> Result<DockerEndpoint, eyre::Report> {
    match endpoint {
        DockerEndpointConfig::Direct {
            cacert,
            client_credentials,
        } => {
            let root_store = {
                let mut store = RootCertStore::empty();

                if let Some(cacert) = cacert {
                    store.add(cacert)?;
                } else {
                    let native_certs = rustls_native_certs::load_native_certs();
                    for error in native_certs.errors {
                        event!(Level::ERROR, ?error, "Failed to load certificate");
                    }

                    for cert in native_certs.certs {
                        store.add(cert).unwrap();
                    }
                }

                store
            };

            let client_config = ClientConfig::builder_with_protocol_versions(DEFAULT_VERSIONS)
                .with_root_certificates(root_store);

            let client_config = if let Some(client_credentials) = client_credentials {
                client_config
                    .with_client_auth_cert(client_credentials.cert_chain, client_credentials.key)?
            } else {
                client_config.with_no_client_auth()
            };

            let connector = HttpsConnectorBuilder::new()
                .with_tls_config(client_config)
                .https_or_http()
                .with_server_name_resolver(FixedServerNameResolver::new(
                    "docker.localhost".try_into().unwrap(),
                ))
                .enable_http1()
                .build();

            Ok(DockerEndpoint::Tls {
                client: build_client(connector),
            })
        },
        DockerEndpointConfig::Socket(socket) => {
            let connector: UnixSocketConnector<PathBuf> = UnixSocketConnector::new(socket);

            Ok(DockerEndpoint::Socket(build_client(connector)))
        },
    }
}

impl DockerHealer {
    pub fn new(
        config: DockerConnection,
        filters: &serde_json::Value,
        webhook_uri: Option<Uri>,
    ) -> Result<Self, eyre::Report> {
        let encoded_filters = url_encode(filters);

        let client = build_docker_client(config.endpoint)?;

        Ok(Self {
            endpoint: client,
            encoded_filters: Rc::from(encoded_filters),
            notifier: WebHookNotifier { uri: webhook_uri },
            timeout_milliseconds: config.timeout_milliseconds,
            uri: config.uri,
        })
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
            Err(eyre::Report::msg(format!(
                "Tried to refresh container but it failed with {:?}",
                status_code
            )))
        }
    }

    async fn send_request(
        &self,
        path_and_query: &str,
        method: Method,
    ) -> Result<Response<Incoming>, eyre::Report> {
        let request = build_request(self.uri.clone(), path_and_query, method)?;

        match self.endpoint {
            DockerEndpoint::Tls { ref client } => {
                let response = execute_request(client, request);

                match timeout(Duration::from_millis(self.timeout_milliseconds), response).await {
                    Ok(Ok(o)) => Ok(o),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(e.into()),
                }
            },
            DockerEndpoint::Socket(ref client) => {
                let response = execute_request(client, request);

                match timeout(Duration::from_millis(self.timeout_milliseconds), response).await {
                    Ok(Ok(o)) => Ok(o),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(e.into()),
                }
            },
        }
    }

    pub async fn check_container_health(
        &self,
        runtime_config: &RuntimeConfig,
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
                        .unwrap_or(runtime_config.default_stop_timeout);

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
                            self.notifier
                                .notify_webhook_success(container_short_id, container_names);
                        },
                        Err(err) => {
                            event!(Level::INFO,
                                error = ?err,
                                "Restarting container {} ({}) failed.",
                                container_names,
                                container_short_id
                            );

                            self.notifier.notify_webhook_failure(
                                container_names,
                                container_short_id,
                                err,
                            );
                        },
                    }
                }
            },
        }
    }
}
