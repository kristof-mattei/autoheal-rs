use std::path::PathBuf;

use color_eyre::eyre;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper_rustls::{FixedServerNameResolver, HttpsConnector, HttpsConnectorBuilder};
use hyper_unix_socket::UnixSocketConnector;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use rustls::client::ClientConfig;
use rustls::pki_types::pem::PemObject as _;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{DEFAULT_VERSIONS, RootCertStore};
use tracing::{Level, event};

use crate::app_config::DockerStartupConfig;
use crate::http_client;

pub struct DockerClient {
    pub endpoint: DockerEndpoint,
    pub uri: http::Uri,
}

pub enum DockerEndpoint {
    Socket(Client<UnixSocketConnector<PathBuf>, Full<Bytes>>),
    Tls {
        client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
    },
}

struct ClientCredentials {
    cert_chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
}

impl DockerClient {
    pub fn build(
        DockerStartupConfig {
            docker_sock: docker_socket_or_uri,
            cacert,
            client_key,
            client_cert,
        }: DockerStartupConfig,
    ) -> Result<DockerClient, eyre::Report> {
        const TCP_START: &str = "tcp://";

        let endpoint = if docker_socket_or_uri.starts_with(TCP_START) {
            let mut docker_socket_or_uri = docker_socket_or_uri;
            docker_socket_or_uri.replace_range(..TCP_START.len(), "https://");

            let client_credentials = match (client_cert, client_key) {
                (Some(client_cert), Some(client_key)) => Some(ClientCredentials {
                    cert_chain: vec![CertificateDer::from_pem_file(client_cert)?],
                    key: PrivateKeyDer::from_pem_file(client_key)?,
                }),
                _ => None,
            };

            let cacert = if let Some(cacert) = cacert {
                Some(CertificateDer::from_pem_file(cacert)?)
            } else {
                None
            };

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

            DockerClient {
                endpoint: DockerEndpoint::Tls {
                    client: http_client::build_client(connector),
                },

                uri: docker_socket_or_uri.parse()?,
            }
        } else {
            // we're connecting over a socket, so the url is localhost

            let connector: UnixSocketConnector<PathBuf> =
                UnixSocketConnector::new(PathBuf::from(docker_socket_or_uri));

            DockerClient {
                endpoint: DockerEndpoint::Socket(http_client::build_client(connector)),
                uri: http::Uri::from_static("http://localhost"),
            }
        };

        Ok(endpoint)
    }
}
