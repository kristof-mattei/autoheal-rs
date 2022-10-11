pub struct ApiConfig {
    pub endpoint: String,
    pub options: Vec<String>,
}

impl ApiConfig {
    // fn curl_options(&self) -> String {
    //     match self {
    //         ApiConfig::Tcp(_) => String::from(
    //             "--cacert /certs/ca.pem --key /certs/client-key.pem --cert /certs/client-cert.pem",
    //         ),
    //         ApiConfig::Socket(s) => format!("--unix-socket {s}"),
    //     }
    // }
}

pub fn build_api_config(mut docker_socket_or_uri: String) -> ApiConfig {
    const TCP_START: &str = "tcp://";
    let endpoint = if docker_socket_or_uri.starts_with(TCP_START) {
        docker_socket_or_uri.replace_range(..TCP_START.len(), "https://");

        let uri: hyper::Uri = docker_socket_or_uri.parse().unwrap();
        uri.to_string()
    } else {
        let socket: hyper::Uri = hyperlocal::Uri::new(docker_socket_or_uri, "").into();
        socket.to_string()
    };

    ApiConfig {
        endpoint,
        options: vec![],
    }
}
