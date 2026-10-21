use std::time::Duration;

use color_eyre::eyre;
use http::Request;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::HeaderValue;
use hyper::{Method, Uri};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use tracing::{Level, event};
use twistlock::models::id::ContainerId;

const WEBHOOK_TIMEOUT: Duration = Duration::from_secs(30);

type WebHookClient = Client<HttpsConnector<HttpConnector>, Full<Bytes>>;

#[derive(Debug)]
struct WebHookInvocation {
    uri: Uri,
    container_name: Box<str>,
    container_short_id: Box<str>,
    outcome: RestartOutcome,
}

impl WebHookInvocation {
    fn to_title(&self) -> &str {
        match self.outcome {
            RestartOutcome::Success => "Container successfully restarted",
            RestartOutcome::Failure(_) => "Container failed to restart",
        }
    }

    fn to_priority(&self) -> usize {
        match self.outcome {
            RestartOutcome::Success => 3,
            RestartOutcome::Failure(_) => 5,
        }
    }

    fn to_tags(&self) -> &str {
        match self.outcome {
            RestartOutcome::Success => "white_check_mark",
            RestartOutcome::Failure(_) => "x",
        }
    }
}

#[derive(Debug)]
pub enum RestartOutcome {
    Success,
    Failure(eyre::Report),
}

struct WebHook {
    uri: Uri,
    client: WebHookClient,
}

pub struct WebHookNotifier {
    webhook: Option<WebHook>,
}

impl WebHookNotifier {
    pub fn new(uri: Option<Uri>) -> Self {
        let webhook = uri.map(|uri| {
            let connector = HttpsConnectorBuilder::new()
                .with_webpki_roots()
                .https_or_http()
                .enable_all_versions()
                .build();

            WebHook {
                uri,
                client: Client::builder(TokioExecutor::new()).build(connector),
            }
        });

        Self { webhook }
    }

    pub fn notify<S: Into<Box<str>>>(
        &self,
        container_id: &ContainerId,
        container_name: S,
        outcome: RestartOutcome,
    ) {
        let Some(ref webhook) = self.webhook else {
            return;
        };

        let invocation = WebHookInvocation {
            uri: webhook.uri.clone(),
            container_name: container_name.into(),
            container_short_id: container_id.as_short().into(),
            outcome,
        };

        let client = webhook.client.clone();

        tokio::task::spawn(async move {
            notify_webhook_and_log(&client, invocation).await;
        });
    }
}

async fn notify_webhook_and_log(client: &WebHookClient, invocation: WebHookInvocation) {
    match notify_webhook(client, &invocation).await {
        Ok(()) => event!(Level::TRACE, ?invocation, "Successfully notified webhook"),
        Err(error) => event!(Level::ERROR, ?error, ?invocation, "Failure sending webhook"),
    }
}

async fn notify_webhook(
    client: &WebHookClient,
    invocation: &WebHookInvocation,
) -> Result<(), eyre::Report> {
    let message = match invocation.outcome {
        RestartOutcome::Success => format!(
            "Container \"{}\" ({}) was unhealthy, but was successfully restarted.",
            invocation.container_name, invocation.container_short_id
        ),
        RestartOutcome::Failure(ref error) => format!(
            "Container \"{}\" ({}) was unhealthy and we failed to restarted it. Please check the logs for more info. \nError: {}",
            invocation.container_name, invocation.container_short_id, error
        ),
    };

    let request = Request::builder()
        .uri(invocation.uri.clone())
        .method(Method::POST)
        .header(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        )
        .header("X-Title", invocation.to_title())
        .header("X-Priority", invocation.to_priority())
        .header("X-Tags", invocation.to_tags())
        .body(Full::new(Bytes::from(message)))?;

    let response = tokio::time::timeout(WEBHOOK_TIMEOUT, client.request(request)).await??;

    if !response.status().is_success() {
        return Err(eyre::eyre!("Webhook responded with {}", response.status()));
    }

    Ok(())
}
