use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre;
use hyper::Uri;

#[derive(Parser, Debug)]
struct RawConfig {
    #[clap(long, env)]
    pub autoheal_container_label: Option<String>,
    #[clap(long, env, default_value_t = 10)]
    pub autoheal_default_stop_timeout: u32,
    #[clap(long, env, default_value_t = 5)]
    pub autoheal_interval: u64,
    #[clap(long, env)]
    pub autoheal_exclude_containers: Vec<String>,
    #[clap(long, env, default_value_t = 0)]
    pub autoheal_start_period: u64,
    #[clap(long, env = "CA")]
    pub cacert: Option<PathBuf>,
    #[clap(long, env)]
    pub client_key: Option<PathBuf>,
    #[clap(long, env)]
    pub client_cert: Option<PathBuf>,
    #[clap(long = "curl_timeout", env, default_value_t = 30000)]
    pub timeout_milliseconds: u64,
    #[clap(long, env, default_value_t = String::from("/var/run/docker.sock"))]
    pub docker_sock: String,
    #[clap(long, env)]
    pub webhook_url: Option<Uri>,
}

pub struct DockerStartupConfig {
    pub docker_sock: String,
    pub cacert: Option<PathBuf>,
    pub client_key: Option<PathBuf>,
    pub client_cert: Option<PathBuf>,
}

pub struct HealerConfig {
    pub default_stop_timeout: u32,
    pub interval: u64,
    pub exclude_containers: Vec<String>,
    pub start_period: u64,
    pub timeout_milliseconds: u64,
}

pub struct AppConfig {
    pub container_label: Option<String>,
    pub docker_startup_config: DockerStartupConfig,
    pub healer_config: HealerConfig,
    pub webhook_url: Option<Uri>,
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, eyre::Report> {
        let raw_config = RawConfig::try_parse()?;

        let docker_startup_config = DockerStartupConfig {
            docker_sock: raw_config.docker_sock,
            cacert: raw_config.cacert,
            client_key: raw_config.client_key,
            client_cert: raw_config.client_cert,
        };

        let healer_config = HealerConfig {
            default_stop_timeout: raw_config.autoheal_default_stop_timeout,
            interval: raw_config.autoheal_interval,
            exclude_containers: raw_config.autoheal_exclude_containers,
            start_period: raw_config.autoheal_start_period,
            timeout_milliseconds: raw_config.timeout_milliseconds,
        };

        Ok(AppConfig {
            docker_startup_config,
            healer_config,
            container_label: raw_config.autoheal_container_label,
            webhook_url: raw_config.webhook_url,
        })
    }
}
