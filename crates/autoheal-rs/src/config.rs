use std::time::Duration;

use tracing::{Level, event};
use twistlock::config::Endpoint;
use url::Url;

pub(crate) struct DockerConfig {
    pub docker_host: Endpoint,
    pub timeout: Duration,
}

pub(crate) struct HealerConfig {
    pub default_stop_timeout: Duration,
    pub interval: Duration,
    pub exclude_containers: Box<[Box<str>]>,
    pub start_period: Duration,
}

pub(crate) struct Config {
    pub container_label: Option<String>,
    pub docker: DockerConfig,
    pub healer: HealerConfig,
    pub webhook_url: Option<Url>,
}

impl Config {
    pub(crate) fn log(&self) {
        event!(Level::INFO, docker_host = %self.docker.docker_host, "Daemon");
    }
}
