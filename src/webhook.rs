use std::collections::HashMap;

use http_body_util::Full;
use http_client::{build_request_with_headers_and_body, send_get_post};
use hyper::body::Bytes;
use hyper::http::HeaderValue;
use hyper::{Method, Uri};
use serde_json::json;

use crate::app_config::AppConfig;
use crate::http_client;

pub fn notify_webhook_success(
    app_config: &AppConfig,
    container_short_id: &str,
    container_name: &str,
) {
    let Some(webhook_url) = app_config.webhook_url.clone() else { return };

    let message = format!("Container {container_name} ({container_short_id}) found to be unhealthy. Successfully restarted the container!");

    tokio::task::spawn(async move {
        notify_webhook_and_log(&webhook_url, message).await;
    });
}

pub fn notify_webhook_failure(
    app_config: &AppConfig,
    container_name: &str,
    container_short_id: &str,
    error: &anyhow::Error,
) {
    let Some(webhook_url) = app_config.webhook_url.clone() else { return };

    let message = format!("Container {container_name} ({container_short_id}) found to be unhealthy. Failed to restart the container! Error: {error:?}");

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

async fn notify_webhook(webhook_url: &Uri, text: &str) -> Result<(), anyhow::Error> {
    let payload = json!({
        "text": text,
    });

    let stream = http_client::connect_tcp_stream(webhook_url)
        .await
        .expect("Couldn't establish connection to webhook_url");

    let data = serde_json::to_string(&payload).expect("Failed to serialize payload");

    // execute webhook requests as background process to prevent healer from blocking
    #[allow(clippy::mutable_key_type)]
    let headers = HashMap::from_iter([(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    )]);

    let request = build_request_with_headers_and_body(
        webhook_url,
        headers,
        Method::POST,
        Full::new(Bytes::from(data)),
    )?;

    send_get_post(stream, request).await?;

    Ok(())
}
