use std::ffi::OsString;

use crate::env::parse_env_variable_with_default;

pub struct DockerConfig {
    pub endpoint: Endpoint,
    pub curl_timeout: u32,
    pub options: Vec<String>,
}

pub enum Endpoint {
    Direct(hyper::Uri),
    Socket(String, hyper::Uri),
}

impl DockerConfig {
    pub fn build() -> Result<DockerConfig, color_eyre::Report> {
        const TCP_START: &str = "tcp://";
        let mut docker_socket_or_uri = std::env::var_os("DOCKER_SOCK")
            .map_or_else(
                || Ok(String::from("/var/run/docker.sock")),
                OsString::into_string,
            )
            .map_err(|err| {
                color_eyre::Report::msg(format!("Could not convert {:?} to String", err))
            })?;

        let curl_timeout = parse_env_variable_with_default("CURL_TIMEOUT", 30)?;

        let endpoint = if docker_socket_or_uri.starts_with(TCP_START) {
            docker_socket_or_uri.replace_range(..TCP_START.len(), "https://");

            Endpoint::Direct(docker_socket_or_uri.parse().unwrap())
        } else {
            // we're connecting over a socket, so the uri is localhost
            Endpoint::Socket(docker_socket_or_uri, "http://localhost".parse().unwrap())
        };

        // TODO check if docker socket exists

        Ok(DockerConfig {
            endpoint,
            curl_timeout,
            options: vec![],
        })
    }

    // fn curl_options(&self) -> String {
    //     match self {
    //         ApiConfig::Tcp(_) => String::from(
    //             "--cacert /certs/ca.pem --key /certs/client-key.pem --cert /certs/client-cert.pem",
    //         ),
    //         ApiConfig::Socket(s) => format!("--unix-socket {}", s),
    //     }
    // }
}
