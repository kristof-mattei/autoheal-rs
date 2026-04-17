use std::time::Duration;

use hashbrown::HashMap;
use http::Uri;
use tokio::time::sleep;
use tracing::{Level, event};
use twistlock::client::Client;
use twistlock::filters::Filters;
use twistlock::models::container::Container;

use crate::config::HealerConfig;
use crate::webhook::WebHookNotifier;

pub struct DockerHealer {
    client: Client,
    filters: Filters,
    healer_config: HealerConfig,
    notifier: WebHookNotifier,
}

impl DockerHealer {
    pub fn new(
        client: Client,
        healer_config: HealerConfig,
        filters: Filters,
        webhook_uri: Option<Uri>,
    ) -> Self {
        Self {
            client,
            filters,
            healer_config,
            notifier: WebHookNotifier { uri: webhook_uri },
        }
    }

    pub async fn check_container_health(&self, container_info: &Container, times: usize) {
        let container_short_id = container_info.get_short_id();

        match container_info.get_name() {
            None => {
                event!(
                    Level::WARN,
                    %container_short_id,
                    "Container name was null, which implies container does not exist - don't restart.",
                );
            },
            Some(container_names) => {
                if &*container_info.state == "restarting" {
                    event!(
                        Level::INFO,
                        %container_names,
                        %container_short_id,
                        "Container found to be restarting - don't restart.",
                    );
                } else {
                    let timeout = container_info.timeout.map_or_else(
                        || self.healer_config.default_stop_timeout,
                        |v| Duration::from_secs(v.into()),
                    );

                    event!(
                        Level::INFO,
                        %container_names,
                        %container_short_id,
                        times_unhealthy = %times,
                        timeout = ?timeout,
                        "Container repeatedly found to be unhealthy. Restarting container now with timeout.",
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
                                Level::WARN,
                                ?error,
                                %container_names,
                                %container_short_id,
                                "Restarting container failed.",
                            );

                            self.notifier.notify_webhook_failure(
                                container_names,
                                container_short_id,
                                error.into(),
                            );
                        },
                    }
                }
            },
        }
    }

    pub async fn monitor_containers(&self) -> ! {
        if self.healer_config.start_period.as_secs() > 0 {
            event!(
                Level::INFO,
                delay = ?self.healer_config.start_period,
                "Monitoring containers for unhealthy status",
            );

            sleep(self.healer_config.start_period).await;
        }

        let mut history_unhealthy = HashMap::<Box<str>, (Option<Box<str>>, usize)>::new();

        loop {
            match self.client.list_containers(&self.filters).await {
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
                                container_names = %container
                                    .get_name()
                                    .as_deref()
                                    .unwrap_or("<UNNAMED CONTAINER>"),
                                container_short_id = %container.get_short_id(),
                                "Container is unhealthy, but it is excluded",
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
                                    container_names = %names.as_deref().unwrap_or("<UNNAMED CONTAINER>"),
                                    container_id = %key,
                                    "Container returned to healthy state.",
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

            tokio::time::sleep(self.healer_config.interval).await;
        }
    }
}
