use std::rc::Rc;
use std::time::Duration;

use color_eyre::eyre;
use hashbrown::HashMap;
use http::Uri;
use http_body_util::BodyExt as _;
use hyper::body::{Buf as _, Incoming};
use hyper::{Method, Response, StatusCode};
use tokio::time::{sleep, timeout};
use tracing::{Level, event};

use crate::app_config::HealerConfig;
use crate::container::Container;
use crate::docker_connection::{DockerClient, DockerEndpoint};
use crate::encoding::url_encode;
use crate::http_client::{build_request, execute_request};
use crate::webhook::WebHookNotifier;

pub struct DockerHealer {
    client: DockerClient,
    encoded_filters: Rc<str>,
    healer_config: HealerConfig,
    notifier: WebHookNotifier,
}

impl DockerHealer {
    pub fn new(
        client: DockerClient,
        healer_config: HealerConfig,
        filters: &serde_json::Value,
        webhook_uri: Option<Uri>,
    ) -> Self {
        let encoded_filters = url_encode(filters);

        Self {
            client,
            encoded_filters: Rc::from(encoded_filters),
            healer_config,
            notifier: WebHookNotifier { uri: webhook_uri },
        }
    }

    pub async fn get_containers(&self) -> Result<Vec<Container>, eyre::Report> {
        let path_and_query = format!("/containers/json?filters={}", self.encoded_filters);

        let response = self.send_request(&path_and_query, Method::GET).await?;

        let reader = response.collect().await?.aggregate().reader();

        let result = serde_json::from_reader::<_, Vec<Container>>(reader)?;

        Ok(result)
    }

    pub async fn restart_container(
        &self,
        container_id: &str,
        timeout: u32,
    ) -> Result<(), eyre::Report> {
        let path_and_query = format!("/containers/{}/restart?t={}", container_id, timeout);

        let response = self.send_request(&path_and_query, Method::POST).await?;

        let status_code = response.status();

        if StatusCode::is_success(&status_code) {
            Ok(())
        } else {
            Err(eyre::Report::msg(format!(
                "Tried to refresh container but it failed with {:?}",
                status_code
            )))
        }
    }

    async fn send_request(
        &self,
        path_and_query: &str,
        method: Method,
    ) -> Result<Response<Incoming>, eyre::Report> {
        let request = build_request(self.client.uri.clone(), path_and_query, method)?;

        match self.client.endpoint {
            DockerEndpoint::Tls(ref client) => {
                let response = execute_request(client, request);

                match timeout(
                    Duration::from_millis(self.healer_config.timeout_milliseconds),
                    response,
                )
                .await
                {
                    Ok(Ok(o)) => Ok(o),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(e.into()),
                }
            },
            DockerEndpoint::Socket(ref client) => {
                let response = execute_request(client, request);

                match timeout(
                    Duration::from_millis(self.healer_config.timeout_milliseconds),
                    response,
                )
                .await
                {
                    Ok(Ok(o)) => Ok(o),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(e.into()),
                }
            },
        }
    }

    pub async fn check_container_health(&self, container_info: &Container, times: usize) {
        let container_short_id = &container_info.id[0..12];

        match container_info.get_name() {
            None => {
                event!(
                    Level::ERROR,
                    "Container name of {} is null, which implies container does not exist - don't restart.",
                    container_short_id
                );
            },
            Some(container_names) => {
                if &*container_info.state == "restarting" {
                    event!(
                        Level::INFO,
                        "Container {} ({}) found to be restarting - don't restart.",
                        container_names,
                        container_short_id
                    );
                } else {
                    let timeout = container_info
                        .timeout
                        .unwrap_or((self.healer_config).default_stop_timeout);

                    event!(
                        Level::INFO,
                        "Container {} ({}) found to be unhealthy {} times. Restarting container now with {}s timeout.",
                        container_names,
                        container_short_id,
                        times,
                        timeout
                    );

                    match self.restart_container(container_short_id, timeout).await {
                        Ok(()) => {
                            self.notifier
                                .notify_webhook_success(container_short_id, container_names);
                        },
                        Err(err) => {
                            event!(Level::INFO,
                                error = ?err,
                                "Restarting container {} ({}) failed.",
                                container_names,
                                container_short_id
                            );

                            self.notifier.notify_webhook_failure(
                                container_names,
                                container_short_id,
                                err,
                            );
                        },
                    }
                }
            },
        }
    }

    pub async fn monitor_containers(&self) -> ! {
        if self.healer_config.start_period > 0 {
            event!(
                Level::INFO,
                "Monitoring containers for unhealthy status in {} second(s)",
                self.healer_config.start_period
            );

            sleep(Duration::from_secs(self.healer_config.start_period)).await;
        }

        let mut history_unhealthy = HashMap::<Rc<str>, (Option<Rc<str>>, usize)>::new();

        loop {
            match self.get_containers().await {
                Ok(containers) => {
                    let mut current_unhealthy: HashMap<Rc<str>, Option<Rc<str>>> = containers
                        .iter()
                        .map(|c| (Rc::clone(&c.id), c.get_name().map(Into::into)))
                        .collect::<HashMap<_, _>>();

                    for container in containers {
                        if container
                            .names
                            .iter()
                            .any(|n| self.healer_config.exclude_containers.contains(n))
                        {
                            event!(
                                Level::INFO,
                                "Container {} ({}) is unhealthy, but it is excluded",
                                container
                                    .get_name()
                                    .as_deref()
                                    .unwrap_or("<UNNAMED CONTAINER>"),
                                &container.id[0..12],
                            );

                            continue;
                        }

                        self.check_container_health(
                            &container,
                            history_unhealthy
                                .get(&container.id)
                                .map_or(1, |&(_, t)| t + 1),
                        )
                        .await;
                    }

                    history_unhealthy = history_unhealthy
                        .into_iter()
                        .filter_map(|(key, (names, times))| {
                            if let Some(new_name) = current_unhealthy.remove(&key) {
                                // still unhealthy
                                // take the new name
                                Some((key, (new_name, times + 1)))
                            } else {
                                // healthy
                                event!(
                                    Level::INFO,
                                    "Container {} ({}) returned to healthy state.",
                                    names.as_deref().unwrap_or("<UNNAMED CONTAINER>"),
                                    key
                                );
                                None
                            }
                        })
                        .collect();
                },
                Err(err) => {
                    event!(Level::ERROR, ?err, "Failed to fetch container info");
                },
            }

            tokio::time::sleep(Duration::from_secs(self.healer_config.interval)).await;
        }
    }
}
