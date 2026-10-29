use std::path::PathBuf;
use std::str::FromStr as _;
use std::time::Duration;

use clap::{Args, Parser};
use twistlock::config::{ClientCredentialPaths, Endpoint, ParseError, Tls};
use url::Url;

use crate::config::{Config, DockerConfig, HealerConfig};

#[derive(Parser, Debug)]
#[command(about, version)]
pub(crate) struct Cli {
    #[arg(
        env,
        default_value = "",
        hide_default_value = true,
        help = "Docker daemon address",
        long,
        value_parser = parse_docker_host
    )]
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
        help = "Timeout for a Docker API call, in seconds, added to the stop timeout for a restart",
        value_parser = parse_duration
    )]
    pub timeout: Duration,

    #[arg(long, env, value_parser = parse_webhook_url)]
    pub webhook_url: Option<Url>,
}

// flattened as `Option<Self>`. clap still marks the non-`Option` fields required, hence `required = false`. The group enforces both-or-neither
#[derive(Args, Debug)]
#[group(requires_all = ["client_key", "client_cert"])]
pub(crate) struct ClientCredentialArgs {
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

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        let docker_host = match (cli.cacert, cli.client_credentials) {
            (None, None) => cli.docker_host,
            (ca_cert, client_credentials) => cli.docker_host.with_tls(Tls {
                verify: true,
                ca_cert,
                client_credentials: client_credentials.map(Into::into),
            }),
        };

        let docker = DockerConfig {
            docker_host,
            timeout: cli.timeout,
        };

        let healer = HealerConfig {
            default_stop_timeout: cli.autoheal_default_stop_timeout,
            interval: cli.autoheal_interval,
            exclude_containers: cli
                .autoheal_exclude_containers
                .into_iter()
                .map(String::into_boxed_str)
                .collect::<Box<[_]>>(),
            start_period: cli.autoheal_start_period,
        };

        Config {
            docker,
            healer,
            container_label: cli.autoheal_container_label,
            webhook_url: cli.webhook_url,
        }
    }
}

fn parse_docker_host(value: &str) -> Result<Endpoint, ParseError> {
    Endpoint::from_str(value)
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    let seconds = value
        .parse()
        .map_err(|error| format!("Could not parse `{}`: {}", value, error))?;

    Ok(Duration::from_secs(seconds))
}

fn parse_webhook_url(value: &str) -> Result<Url, String> {
    let url =
        Url::parse(value).map_err(|error| format!("Could not parse `{}`: {}", value, error))?;

    match url.scheme() {
        "http" | "https" => Ok(url),
        scheme => Err(format!(
            "Unsupported scheme `{}`, expected `http` or `https`",
            scheme
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::Parser as _;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    use super::Cli;

    // `--docker-host` prevents the tests from parsing `DOCKER_HOST`
    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(
            ["autoheal-rs", "--docker-host", "tcp://127.0.0.1:2375"]
                .into_iter()
                .chain(args.iter().copied()),
        )
    }

    #[test]
    fn client_credentials_absent() {
        let config = parse(&[]).unwrap();

        assert!(config.client_credentials.is_none());
    }

    #[test]
    fn client_credentials_both_present() {
        let config = parse(&[
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
        let error = parse(&["--client-key", "/certs/key.pem"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn client_cert_without_client_key_is_rejected() {
        let error = parse(&["--client-cert", "/certs/cert.pem"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn webhook_url_with_http_scheme() {
        let config = parse(&["--webhook-url", "http://ntfy.example/autoheal"]).unwrap();

        assert_eq!(
            config.webhook_url.unwrap().as_str(),
            "http://ntfy.example/autoheal"
        );
    }

    #[test]
    fn webhook_url_with_https_scheme() {
        let config = parse(&["--webhook-url", "https://ntfy.example/autoheal"]).unwrap();

        assert_eq!(
            config.webhook_url.unwrap().as_str(),
            "https://ntfy.example/autoheal"
        );
    }

    #[test]
    fn webhook_url_with_other_scheme_is_rejected() {
        let error = parse(&["--webhook-url", "ftp://ntfy.example/autoheal"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn webhook_url_without_scheme_is_rejected() {
        let error = parse(&["--webhook-url", "ntfy.example/autoheal"]).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
    }
}
