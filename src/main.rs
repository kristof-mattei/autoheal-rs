mod app_config;
mod docker;
mod docker_healer;
mod encoding;
mod ffi_handlers;
mod helpers;
mod http_client;
mod unhealthy_filters;
mod utils;
mod webhook;

use std::convert::Infallible;
use std::env;
use std::env::VarError;

use app_config::AppConfig;
use color_eyre::config::HookBuilder;
use color_eyre::eyre;
use docker::client::DockerClient;
use docker_healer::DockerHealer;
use ffi_handlers::set_up_handlers;
use tracing::{Level, event};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::{EnvFilter, Layer as _};

use crate::utils::flatten_handle;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn build_default_filter() -> EnvFilter {
    EnvFilter::builder()
        .parse(format!("INFO,{}=TRACE", env!("CARGO_CRATE_NAME")))
        .expect("Default filter should always work")
}

fn init_tracing() -> Result<(), eyre::Report> {
    let (filter, filter_parsing_error) = match env::var(EnvFilter::DEFAULT_ENV) {
        Ok(user_directive) => match EnvFilter::builder().parse(user_directive) {
            Ok(filter) => (filter, None),
            Err(error) => (build_default_filter(), Some(eyre::Report::new(error))),
        },
        Err(VarError::NotPresent) => (build_default_filter(), None),
        Err(error @ VarError::NotUnicode(_)) => {
            (build_default_filter(), Some(eyre::Report::new(error)))
        },
    };

    let registry = tracing_subscriber::registry();

    #[cfg(feature = "tokio-console")]
    let registry = registry.with(console_subscriber::ConsoleLayer::builder().spawn());

    registry
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .with(tracing_error::ErrorLayer::default())
        .try_init()?;

    filter_parsing_error.map_or(Ok(()), Err)
}

fn main() -> Result<Infallible, eyre::Report> {
    HookBuilder::default()
        .capture_span_trace_by_default(true)
        .display_env_section(false)
        .install()?;

    init_tracing()?;

    set_up_handlers()?;

    // initialize the runtime
    let result: Result<Infallible, eyre::Report> = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("Failed building the Runtime")
        .block_on(async {
            // explicitly launch everything in a spawned task
            // see https://docs.rs/tokio/latest/tokio/attr.main.html#non-worker-async-function
            let handle = tokio::task::spawn(healer());

            flatten_handle(handle).await
        });

    result
}

async fn healer() -> Result<Infallible, eyre::Report> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    event!(
        Level::INFO,
        "{} v{} - built for {}-{}",
        name,
        version,
        std::env::var("TARGETARCH")
            .as_deref()
            .unwrap_or("unknown-arch"),
        std::env::var("TARGETVARIANT")
            .as_deref()
            .unwrap_or("base variant")
    );

    let AppConfig {
        docker_startup_config,
        healer_config,
        container_label,
        webhook_url,
    } = AppConfig::build()?;

    let filters = unhealthy_filters::build(container_label.as_deref());

    let docker_client = DockerClient::build(docker_startup_config)?;

    let docker_healer = DockerHealer::new(docker_client, healer_config, &filters, webhook_url);

    // TODO define failure mode
    // Do we fail? Do we retry?
    docker_healer.monitor_containers().await;
}

#[cfg(test)]
mod tests {}
