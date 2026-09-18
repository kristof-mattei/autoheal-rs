use color_eyre::eyre;
use http::{Request, Response};
use http_body_util::Full;
use hyper::body::{Body, Bytes};
use hyper::http::HeaderValue;
use hyper::{Method, Uri};
use hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::Connect;
use hyper_util::rt::TokioExecutor;
use tracing::{Level, event};
use twistlock::models::id::ContainerId;

/// Executes a request on a client.
///
/// # Errors
///
/// When the request errors.
pub async fn execute_request<C, B>(
    client: &Client<C, B>,
    request: Request<B>,
) -> Result<Response<hyper::body::Incoming>, hyper_util::client::legacy::Error>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: Body + Send + 'static + Unpin,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let response = client.request(request).await?;

    Ok(response)
}

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

pub struct WebHookNotifier {
    pub uri: Option<Uri>,
}

impl WebHookNotifier {
    pub fn notify<S: Into<Box<str>>>(
        &self,
        container_id: &ContainerId,
        container_name: S,
        outcome: RestartOutcome,
    ) {
        let Some(uri) = self.uri.clone() else {
            return;
        };

        let invocation = WebHookInvocation {
            uri,
            container_name: container_name.into(),
            container_short_id: container_id.as_short().into(),
            outcome,
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
            HeaderValue::from_static("application/json"),
        )
        .header("X-Title", invocation.to_title())
        .header("X-Priority", invocation.to_priority())
        .header("X-Tags", invocation.to_tags())
        .body(Full::new(Bytes::from(message)))?;

    let client = Client::builder(TokioExecutor::new()).build(connector);

    execute_request(&client, request)
        .await
        .map(|_| ())
        .map_err(Into::into)
}
