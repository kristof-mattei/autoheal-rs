use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre;
use hyper::Uri;

#[derive(Parser, Debug)]
pub struct RawConfig {
    #[clap(long, env)]
    pub autoheal_container_label: Option<String>,
    #[clap(long, env)]
    pub autoheal_default_stop_timeout: u32,
    #[clap(long, env)]
    pub autoheal_interval: u64,
    #[clap(long, env)]
    pub autoheal_exclude_containers: Vec<String>,
    #[clap(long, env)]
    pub autoheal_start_period: u64,
    #[clap(long, env = "CA")]
    pub cacert: Option<PathBuf>,
    #[clap(long, env)]
    pub client_key: Option<PathBuf>,
    #[clap(long, env)]
    pub client_cert: Option<PathBuf>,
    #[clap(long, env, default_value_t = 30000)]
    pub curl_timeout: u64,
    #[clap(long, env, default_value_t = String::from("/var/run/docker.sock"))]
    pub docker_sock: String,
    #[clap(long, env)]
    pub webhook_url: Option<Uri>,
}

pub struct DockerStartupConfig {
    pub curl_timeout: u64,
    pub docker_sock: String,
    pub cacert: Option<PathBuf>,
    pub client_key: Option<PathBuf>,
    pub client_cert: Option<PathBuf>,
}

pub struct RuntimeConfig {
    pub container_label: Option<String>,
    pub default_stop_timeout: u32,
    pub interval: u64,
    pub exclude_containers: Vec<String>,
    pub start_period: u64,
}

impl RawConfig {
    pub fn build() -> Result<(DockerStartupConfig, RuntimeConfig, Option<Uri>), eyre::Report> {
        let app_config = RawConfig::try_parse()?;

        let docker_startup_config = DockerStartupConfig {
            docker_sock: app_config.docker_sock,
            curl_timeout: app_config.curl_timeout,
            cacert: app_config.cacert,
            client_key: app_config.client_key,
            client_cert: app_config.client_cert,
        };

        let other_config = RuntimeConfig {
            container_label: app_config.autoheal_container_label,
            default_stop_timeout: app_config.autoheal_default_stop_timeout,
            interval: app_config.autoheal_interval,
            exclude_containers: app_config.autoheal_exclude_containers,
            start_period: app_config.autoheal_start_period,
        };

        Ok((docker_startup_config, other_config, app_config.webhook_url))
    }
}
