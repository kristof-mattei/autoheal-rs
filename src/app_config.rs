use crate::env::{parse_env_variable, parse_env_variable_with_default};
use hyper::Uri;

pub struct AppConfig {
    pub webhook_url: Option<Uri>,
    pub autoheal_container_label: String,
    pub autoheal_start_period: u64,
    pub autoheal_interval: u64,
    pub autoheal_default_stop_timeout: u32,
}

pub fn build() -> Result<AppConfig, anyhow::Error> {
    Ok(AppConfig {
        webhook_url: parse_env_variable("WEBHOOK_URL")?,
        autoheal_container_label: parse_env_variable_with_default(
            "AUTOHEAL_CONTAINER_LABEL",
            String::from("autoheal"),
        )?,
        autoheal_start_period: parse_env_variable_with_default("AUTOHEAL_START_PERIOD", 0)?,
        autoheal_interval: parse_env_variable_with_default("AUTOHEAL_INTERVAL", 5)?,
        autoheal_default_stop_timeout: parse_env_variable_with_default(
            "AUTOHEAL_DEFAULT_STOP_TIMEOUT",
            10,
        )?,
    })
}
