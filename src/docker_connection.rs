use std::path::PathBuf;

use color_eyre::eyre;
use rustls::pki_types::pem::PemObject as _;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

use crate::app_config::DockerStartupConfig;

pub struct DockerConfig {
    pub endpoint: DockerEndpointConfig,
    pub uri: http::Uri,
}

pub enum DockerEndpointConfig {
    Direct {
        client_credentials: Option<ClientCredentials>,
        cacert: Option<CertificateDer<'static>>,
    },
    Socket(PathBuf),
}

pub struct ClientCredentials {
    pub cert_chain: Vec<CertificateDer<'static>>,
    pub key: PrivateKeyDer<'static>,
}

impl DockerConfig {
    pub fn build(
        DockerStartupConfig {
            docker_sock: docker_socket_or_uri,
            cacert,
            client_key,
            client_cert,
        }: DockerStartupConfig,
    ) -> Result<DockerConfig, eyre::Report> {
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

            DockerConfig {
                endpoint: DockerEndpointConfig::Direct {
                    client_credentials,
                    cacert,
                },
                uri: docker_socket_or_uri.parse()?,
            }
        } else {
            // we're connecting over a socket, so the url is localhost
            DockerConfig {
                endpoint: DockerEndpointConfig::Socket(PathBuf::from(docker_socket_or_uri)),
                uri: http::Uri::from_static("http://localhost"),
            }
        };

        Ok(endpoint)
    }
}
