use color_eyre::eyre;
use http::Request;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::HeaderValue;
use hyper::{Method, Uri};
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use tracing::{Level, event};

use crate::http_client::execute_request;

#[derive(Debug)]
struct WebHookInvocation {
    uri: Uri,
    container_name: String,
    container_short_id: String,
    state: State,
}

impl WebHookInvocation {
    fn to_title(&self) -> &str {
        match self.state {
            State::Success => "Container successfully restarted",
            State::Failure(_) => "Container failed to restart",
        }
    }

    fn to_priority(&self) -> usize {
        match self.state {
            State::Success => 3,
            State::Failure(_) => 5,
        }
    }

    fn to_tags(&self) -> &str {
        match self.state {
            State::Success => "white_check_mark",
            State::Failure(_) => "x",
        }
    }
}

#[derive(Debug)]
enum State {
    Success,
    Failure(eyre::Report),
}

pub struct WebHookNotifier {
    pub uri: Option<Uri>,
}

impl WebHookNotifier {
    pub fn notify_webhook_success<S1: Into<String>, S2: Into<String>>(
        &self,
        container_short_id: S1,
        container_name: S2,
    ) {
        let Some(uri) = self.uri.clone() else {
            return;
        };

        let invocation = WebHookInvocation {
            uri,
            container_name: container_name.into(),
            container_short_id: container_short_id.into(),
            state: State::Success,
        };

        tokio::task::spawn(async move {
            notify_webhook_and_log(invocation).await;
        });
    }

    pub fn notify_webhook_failure<S1: Into<String>, S2: Into<String>>(
        &self,
        container_name: S1,
        container_short_id: S2,
        error: eyre::Report,
    ) {
        let Some(uri) = self.uri.clone() else {
            return;
        };

        let invocation = WebHookInvocation {
            uri,
            container_name: container_name.into(),
            container_short_id: container_short_id.into(),
            state: State::Failure(error),
        };

        tokio::task::spawn(async move {
            notify_webhook_and_log(invocation).await;
        });
    }
}

async fn notify_webhook_and_log(invocation: WebHookInvocation) {
    match notify_webhook(&invocation).await {
        Ok(()) => event!(Level::TRACE, ?invocation, "Successfully notified webhook"),
        Err(error) => event!(Level::TRACE, ?error, ?invocation, "Failure sending webhook"),
    }
}

async fn notify_webhook(invocation: &WebHookInvocation) -> Result<(), eyre::Report> {
    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_all_versions()
        .build();

    let client = Client::builder(TokioExecutor::new()).build(connector);

    let message = match invocation.state {
        State::Success => format!(
            "Container \"{}\" ({}) was unhealthy, but was successfully restarted.",
            invocation.container_name, invocation.container_short_id
        ),
        State::Failure(ref error) => format!(
            "Container \"{}\" ({}) was unhealthy and we failed to restarted it. Please check the logs for more info. \nError: {}",
            invocation.container_name, invocation.container_short_id, error
        ),
    };

    let request = Request::builder()
        .uri(invocation.uri.clone())
        .method(Method::POST)
        .header(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )
        .header("X-Title", invocation.to_title())
        .header("X-Priority", invocation.to_priority())
        .header("X-Tags", invocation.to_tags())
        .body(Full::new(Bytes::from(message)))?;

    execute_request(&client, request).await.map(|_| ())
}
