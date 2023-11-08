use color_eyre::eyre::Context;
use http::Request;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::HeaderValue;
use hyper::{Method, Uri};
use hyper_tls::HttpsConnector;
use serde_json::json;

use crate::app_config::AppConfig;
use crate::http_client::execute_request;

pub fn notify_webhook_success(
    app_config: &AppConfig,
    container_short_id: &str,
    container_name: &str,
) {
    let Some(webhook_url) = app_config.webhook_url.clone() else {
        return;
    };

    let message = format!(
        "Container \"{}\" ({}) found to be unhealthy. Successfully restarted the container!",
        container_name, container_short_id
    );

    tokio::task::spawn(async move {
        notify_webhook_and_log(&webhook_url, message).await;
    });
}

pub fn notify_webhook_failure(
    app_config: &AppConfig,
    container_name: &str,
    container_short_id: &str,
    error: &color_eyre::Report,
) {
    let Some(webhook_url) = app_config.webhook_url.clone() else {
        return;
    };

    let message = format!(
        "Container \"{}\" ({}) found to be unhealthy. Failed to restart the container! Error: {:?}",
        container_name, container_short_id, error
    );

    tokio::task::spawn(async move {
        notify_webhook_and_log(&webhook_url, message).await;
    });
}

async fn notify_webhook_and_log(webhook_url: &Uri, text: String) {
    match notify_webhook(webhook_url, &text).await {
        Ok(()) => tracing::info!(
            message = "Successfully notified webhook",
            url = ?webhook_url,
            text = text
        ),
        Err(e) => tracing::error!(
            message = "Failure sending webhook",
            url = ?webhook_url,
            text = text,
            error = ?e
        ),
    };
}

async fn notify_webhook(webhook_url: &Uri, text: &str) -> Result<(), color_eyre::Report> {
    let connector = HttpsConnector::new();

    let payload = json!({
        "text": text,
    });

    let data = serde_json::to_string(&payload).wrap_err_with(|| "Failed to serialize payload")?;

    let request = Request::builder()
        .uri(webhook_url)
        .method(Method::POST)
        .header(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )
        .body(Full::new(Bytes::from(data)))?;

    execute_request(connector, request).await?;

    Ok(())
}
