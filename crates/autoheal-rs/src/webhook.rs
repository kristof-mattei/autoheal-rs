use std::time::Duration;

use color_eyre::eyre;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Url};
use tracing::{Level, event};
use twistlock::models::id::ContainerId;

const WEBHOOK_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
struct WebHookInvocation {
    url: Url,
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
    url: Url,
    client: Client,
}

pub struct WebHookNotifier {
    webhook: Option<WebHook>,
}

impl WebHookNotifier {
    pub fn new(url: Option<Url>) -> Result<Self, reqwest::Error> {
        let Some(url) = url else {
            return Ok(Self { webhook: None });
        };

        let client = Client::builder().timeout(WEBHOOK_TIMEOUT).build()?;

        Ok(Self {
            webhook: Some(WebHook { url, client }),
        })
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
            url: webhook.url.clone(),
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

async fn notify_webhook_and_log(client: &Client, invocation: WebHookInvocation) {
    match notify_webhook(client, &invocation).await {
        Ok(()) => event!(Level::TRACE, ?invocation, "Successfully notified webhook"),
        Err(error) => event!(Level::ERROR, ?error, ?invocation, "Failure sending webhook"),
    }
}

async fn notify_webhook(
    client: &Client,
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

    let response = client
        .post(invocation.url.clone())
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .header("X-Title", invocation.to_title())
        .header("X-Priority", invocation.to_priority())
        .header("X-Tags", invocation.to_tags())
        .body(message)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(eyre::eyre!("Webhook responded with {}", response.status()));
    }

    Ok(())
}
