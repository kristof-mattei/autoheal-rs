use std::path::PathBuf;
use std::str::FromStr as _;
use std::time::Duration;

use clap::{Args, Parser};
use color_eyre::eyre;
use hyper::Uri;
use tracing::{Level, event};
use twistlock::client::ClientCredentialPaths;
use twistlock::config::Endpoint;

const DEFAULT_DOCKER_HOST: &str = "/var/run/docker.sock";

#[derive(Parser, Debug)]
struct RawConfig {
    #[arg(env, default_value = DEFAULT_DOCKER_HOST, value_parser = parse_docker_host, help = "Path to docker TCP/UNIX socket", long="docker")]
    pub docker_host: Endpoint,

    #[arg(long, env)]
    pub autoheal_container_label: Option<String>,

    #[arg(
        env,
        default_value = "10",
        long,
        help = "When container is unhealthy, how long to wait for it to stop, before forcefully restarting it, in seconds",
        value_parser = parse_duration
    )]
    pub autoheal_default_stop_timeout: Duration,

    #[arg(
        env,
        default_value = "5",
        long,
        help = "Interval between checks, in seconds",
        value_parser = parse_duration
    )]
    pub autoheal_interval: Duration,

    #[arg(long, env)]
    pub autoheal_exclude_containers: Vec<String>,

    #[arg(
        env,
        default_value = "0",
        help = "Startup timeout, in seconds",
        value_parser = parse_duration,
        long,
   )]
    pub autoheal_start_period: Duration,

    #[arg(long, env = "CA")]
    pub cacert: Option<PathBuf>,

    #[command(flatten)]
    pub client_credentials: Option<ClientCredentialArgs>,

    #[arg(
        env = "timeout",
        default_value = "30",
        long,
        help = "Docker socket timeout, in seconds, only used when connecting over tcp",
        value_parser = parse_duration
    )]
    pub timeout: Duration,

    #[arg(long, env)]
    pub webhook_url: Option<Uri>,
}

// flattened as `Option<Self>`. clap still marks the non-`Option` fields required, hence `required = false`. The group enforces both-or-neither
#[derive(Args, Debug)]
#[group(requires_all = ["client_key", "client_cert"])]
struct ClientCredentialArgs {
    #[arg(
        long,
        env,
        required = false,
        help = "Path to the client private key for mutual TLS with the Docker daemon"
    )]
    pub client_key: PathBuf,

    #[arg(
        long,
        env,
        required = false,
        help = "Path to the client certificate for mutual TLS with the Docker daemon"
    )]
    pub client_cert: PathBuf,
}

impl From<ClientCredentialArgs> for ClientCredentialPaths {
    fn from(args: ClientCredentialArgs) -> Self {
        ClientCredentialPaths {
            key: args.client_key,
            cert: args.client_cert,
        }
    }
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
    pub client_credentials: Option<ClientCredentialPaths>,
    pub timeout: Duration,
}

pub struct HealerConfig {
    pub default_stop_timeout: Duration,
    pub interval: Duration,
    pub exclude_containers: Box<[Box<str>]>,
    pub start_period: Duration,
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
            client_credentials: raw_config.client_credentials.map(Into::into),
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::Parser as _;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    use super::RawConfig;

    #[test]
    fn client_credentials_absent() {
        let config = RawConfig::try_parse_from(["autoheal-rs"]).unwrap();

        assert!(config.client_credentials.is_none());
    }

    #[test]
    fn client_credentials_both_present() {
        let config = RawConfig::try_parse_from([
            "autoheal-rs",
            "--client-key",
            "/certs/key.pem",
            "--client-cert",
            "/certs/cert.pem",
        ])
        .unwrap();

        let credentials = config.client_credentials.unwrap();

        assert_eq!(credentials.client_key, Path::new("/certs/key.pem"));
        assert_eq!(credentials.client_cert, Path::new("/certs/cert.pem"));
    }

    #[test]
    fn client_key_without_client_cert_is_rejected() {
        let error = RawConfig::try_parse_from(["autoheal-rs", "--client-key", "/certs/key.pem"])
            .unwrap_err();

        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn client_cert_without_client_key_is_rejected() {
        let error = RawConfig::try_parse_from(["autoheal-rs", "--client-cert", "/certs/cert.pem"])
            .unwrap_err();

        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
    }
}
