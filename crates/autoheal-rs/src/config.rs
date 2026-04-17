use std::path::PathBuf;
use std::str::FromStr as _;
use std::time::Duration;

use clap::Parser;
use color_eyre::eyre;
use hyper::Uri;
use tracing::{Level, event};
use twistlock::config::Endpoint;

const DEFAULT_DOCKER_HOST: &str = "/var/run/docker.sock";

#[derive(Parser, Debug)]
struct RawConfig {
    #[arg(env, default_value = DEFAULT_DOCKER_HOST, value_parser = parse_docker_host, help = "Path to docker TCP/UNIX socket", long="docker")]
    pub docker_host: Endpoint,
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
    #[arg(
        env = "timeout",
        default_value = "30",
        long,
        help = "Docker socket timeout, in seconds, only used when connecting over tcp",
        value_parser = parse_duration
    )]
    pub timeout: Duration,
    #[clap(long, env)]
    pub webhook_url: Option<Uri>,
}

impl RawConfig {
    pub fn print(&self) {
        event!(Level::INFO, docker_host = %self.docker_host, "Daemon");
    }
}

fn parse_docker_host(value: &str) -> Result<Endpoint, String> {
    Endpoint::from_str(value)
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    let seconds = value
        .parse()
        .map_err(|error| format!("Could not parse `{}`: {}", value, error))?;

    Ok(Duration::from_secs(seconds))
}

pub struct DockerConfig {
    pub docker_host: Endpoint,
    pub cacert: Option<PathBuf>,
    pub client_key: Option<PathBuf>,
    pub client_cert: Option<PathBuf>,
    pub timeout: Duration,
}

pub struct HealerConfig {
    pub default_stop_timeout: u32,
    pub interval: u64,
    pub exclude_containers: Box<[Box<str>]>,
    pub start_period: u64,
}

pub struct AppConfig {
    pub container_label: Option<String>,
    pub docker_config: DockerConfig,
    pub healer_config: HealerConfig,
    pub webhook_url: Option<Uri>,
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, eyre::Report> {
        let raw_config = RawConfig::try_parse()?;

        raw_config.print();

        let docker_config = DockerConfig {
            docker_host: raw_config.docker_host,
            cacert: raw_config.cacert,
            client_key: raw_config.client_key,
            client_cert: raw_config.client_cert,
            timeout: raw_config.timeout,
        };

        let healer_config = HealerConfig {
            default_stop_timeout: raw_config.autoheal_default_stop_timeout,
            interval: raw_config.autoheal_interval,
            exclude_containers: raw_config
                .autoheal_exclude_containers
                .into_iter()
                .map(String::into_boxed_str)
                .collect::<Box<[_]>>(),
            start_period: raw_config.autoheal_start_period,
        };

        Ok(AppConfig {
            docker_config,
            healer_config,
            container_label: raw_config.autoheal_container_label,
            webhook_url: raw_config.webhook_url,
        })
    }
}
