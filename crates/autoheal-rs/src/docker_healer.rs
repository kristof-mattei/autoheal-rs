use std::time::Duration;

use app_config::HealerConfig;
use hashbrown::HashMap;
use http::Uri;
use shared::docker::client::DockerClient;
use shared::docker::container::Container;
use tokio::time::sleep;
use tracing::{Level, event};

use crate::app_config;
use crate::encoding::url_encode;
use crate::webhook::WebHookNotifier;

pub struct DockerHealer {
    client: DockerClient,
    encoded_filters: Box<str>,
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
            encoded_filters: encoded_filters.into_boxed_str(),
            healer_config,
            notifier: WebHookNotifier { uri: webhook_uri },
        }
    }

    pub async fn check_container_health(&self, container_info: &Container, times: usize) {
        let container_short_id = container_info.get_short_id();

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

                    match self
                        .client
                        .restart_container(container_short_id, timeout)
                        .await
                    {
                        Ok(()) => {
                            self.notifier
                                .notify_webhook_success(container_short_id, container_names);
                        },
                        Err(error) => {
                            event!(
                                Level::INFO,
                                ?error,
                                "Restarting container {} ({}) failed.",
                                container_names,
                                container_short_id
                            );

                            self.notifier.notify_webhook_failure(
                                container_names,
                                container_short_id,
                                error,
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

        let mut history_unhealthy = HashMap::<Box<str>, (Option<Box<str>>, usize)>::new();

        loop {
            match self.client.get_containers(&self.encoded_filters).await {
                Ok(containers) => {
                    let mut current_unhealthy: HashMap<Box<str>, Option<Box<str>>> = containers
                        .iter()
                        .map(|c| (Box::clone(&c.id), c.get_name().map(Into::into)))
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
                                container.get_short_id(),
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
                Err(error) => {
                    event!(Level::ERROR, ?error, "Failed to fetch container info");
                },
            }

            tokio::time::sleep(Duration::from_secs(self.healer_config.interval)).await;
        }
    }
}
