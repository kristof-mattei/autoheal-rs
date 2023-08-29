use hyper::Uri;

use crate::env::{try_parse_env_variable_with_default, try_parse_optional_env_variable};

pub struct AppConfig {
    pub webhook_url: Option<Uri>,
    pub autoheal_container_label: String,
    pub autoheal_start_period: u64,
    pub autoheal_interval: u64,
    pub autoheal_default_stop_timeout: u32,
}

impl AppConfig {
    pub fn build() -> Result<AppConfig, color_eyre::Report> {
        Ok(AppConfig {
            webhook_url: try_parse_optional_env_variable("WEBHOOK_URL")?,
            autoheal_container_label: try_parse_env_variable_with_default(
                "AUTOHEAL_CONTAINER_LABEL",
                String::from("autoheal"),
            )?,
            autoheal_start_period: try_parse_env_variable_with_default("AUTOHEAL_START_PERIOD", 0)?,
            autoheal_interval: try_parse_env_variable_with_default("AUTOHEAL_INTERVAL", 5)?,
            autoheal_default_stop_timeout: try_parse_env_variable_with_default(
                "AUTOHEAL_DEFAULT_STOP_TIMEOUT",
                10,
            )?,
        })
    }
}
