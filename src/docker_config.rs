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
    pub fn new(mut docker_socket_or_uri: String, curl_timeout: u32) -> Self {
        const TCP_START: &str = "tcp://";
        let endpoint = if docker_socket_or_uri.starts_with(TCP_START) {
            docker_socket_or_uri.replace_range(..TCP_START.len(), "https://");

            Endpoint::Direct(docker_socket_or_uri.parse().unwrap())
        } else {
            // we're connecting over a socket, so the uri is localhost
            Endpoint::Socket(docker_socket_or_uri, "http://localhost".parse().unwrap())
        };

        Self {
            endpoint,
            curl_timeout,
            options: vec![],
        }
    }

    // fn curl_options(&self) -> String {
    //     match self {
    //         ApiConfig::Tcp(_) => String::from(
    //             "--cacert /certs/ca.pem --key /certs/client-key.pem --cert /certs/client-cert.pem",
    //         ),
    //         ApiConfig::Socket(s) => format!("--unix-socket {s}"),
    //     }
    // }
}

pub fn build() -> Result<DockerConfig, anyhow::Error> {
    let docker_sock = std::env::var_os("DOCKER_SOCK")
        .map_or_else(
            || Ok(String::from("/var/run/docker.sock")),
            OsString::into_string,
        )
        .map_err(|err| anyhow::Error::msg(format!("Could not convert {:?} to String", err)))?;

    Ok(DockerConfig::new(
        docker_sock,
        parse_env_variable_with_default("CURL_TIMEOUT", 30)?,
    ))
}
