use std::path::PathBuf;
use std::time::Duration;

use clap::Parser as _;
use color_eyre::eyre;
use hyper::Uri;
use tracing::{Level, event};
use twistlock::client::ClientCredentialPaths;
use twistlock::config::Endpoint;

use crate::cli::Cli;

pub struct DockerConfig {
    pub docker_host: Endpoint,
    pub cacert: Option<PathBuf>,
    pub client_credentials: Option<ClientCredentialPaths>,
    pub timeout: Duration,
}

pub struct HealerConfig {
    pub default_stop_timeout: Duration,
    pub interval: Duration,
    pub exclude_containers: Box<[Box<str>]>,
    pub start_period: Duration,
}

pub struct Config {
    pub container_label: Option<String>,
    pub docker: DockerConfig,
    pub healer: HealerConfig,
    pub webhook_url: Option<Uri>,
}

impl Config {
    pub fn build() -> Result<Config, eyre::Report> {
        let config = Config::from(Cli::try_parse()?);

        config.log();

        Ok(config)
    }

    pub fn log(&self) {
        event!(Level::INFO, docker_host = %self.docker.docker_host, "Daemon");
    }
}
