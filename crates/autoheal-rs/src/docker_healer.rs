use std::time::Duration;

use color_eyre::eyre;
use hashbrown::HashMap;
use tokio::time::{MissedTickBehavior, sleep, timeout};
use tracing::{Level, event};
use twistlock::client::Client;
use twistlock::endpoint::ApiEndpointCallError;
use twistlock::filters::{Filters, Status};
use twistlock::models::container::{ContainerSummary, StopTimeout};
use twistlock::models::id::ContainerId;

use crate::config::HealerConfig;
use crate::webhook::{RestartOutcome, WebHookNotifier};

pub(crate) struct DockerHealer {
    client: Client,
    docker_timeout: Duration,
    filters: Filters,
    healer_config: HealerConfig,
    notifier: WebHookNotifier,
}

async fn with_timeout<T, F>(limit: Duration, call: F) -> Result<T, eyre::Report>
where
    F: Future<Output = Result<T, ApiEndpointCallError>>,
{
    match timeout(limit, call).await {
        Ok(result) => Ok(result?),
        Err(elapsed) => Err(eyre::Report::new(elapsed).wrap_err(format!(
            "The Docker daemon did not answer within {:?}",
            limit
        ))),
    }
}

fn get_timeout(labels: &HashMap<Box<str>, Box<str>>) -> Option<Duration> {
    if let Some(timeout) = labels.get("autoheal.stop.timeout") {
        match humantime::parse_duration(timeout) {
            Ok(value) => Some(value),
            Err(error) => {
                event!(
                    Level::WARN,
                    ?timeout,
                    ?error,
                    "Could not parse timeout as a duration"
                );

                None
            },
        }
    } else {
        None
    }
}

impl DockerHealer {
    pub(crate) fn new(
        client: Client,
        docker_timeout: Duration,
        healer_config: HealerConfig,
        filters: Filters,
        notifier: WebHookNotifier,
    ) -> Self {
        Self {
            client,
            docker_timeout,
            filters,
            healer_config,
            notifier,
        }
    }

    pub(crate) async fn check_container_health(
        &self,
        container_info: &ContainerSummary,
        times: usize,
    ) {
        let container_short_id = container_info.get_short_id();

        match container_info.get_name() {
            None => {
                event!(
                    Level::WARN,
                    %container_short_id,
                    "Container name was null, which implies container does not exist - don't restart.",
                );
            },
            Some(container_name) => {
                if container_info.state == Status::Restarting {
                    event!(
                        Level::INFO,
                        %container_name,
                        %container_short_id,
                        "Container found to be restarting - don't restart.",
                    );
                } else {
                    let timeout = get_timeout(&container_info.labels)
                        .unwrap_or(self.healer_config.default_stop_timeout);

                    event!(
                        Level::INFO,
                        %container_name,
                        %container_short_id,
                        times_unhealthy = %times,
                        timeout = ?timeout,
                        "Container repeatedly found to be unhealthy. Restarting container now with timeout.",
                    );

                    // the daemon answers a restart once the container has stopped
                    match with_timeout(
                        timeout.saturating_add(self.docker_timeout),
                        self.client
                            .restart_container(&container_info.id, StopTimeout::After(timeout)),
                    )
                    .await
                    {
                        Ok(()) => {
                            self.notifier.notify(
                                &container_info.id,
                                container_name,
                                RestartOutcome::Success,
                            );
                        },
                        Err(error) => {
                            event!(
                                Level::WARN,
                                ?error,
                                %container_name,
                                %container_short_id,
                                "Restarting container failed.",
                            );

                            self.notifier.notify(
                                &container_info.id,
                                container_name,
                                RestartOutcome::Failure(error),
                            );
                        },
                    }
                }
            },
        }
    }

    pub(crate) async fn monitor_containers(&self) -> ! {
        if !self.healer_config.start_period.is_zero() {
            event!(
                Level::INFO,
                delay = ?self.healer_config.start_period,
                "Monitoring containers for unhealthy status",
            );

            sleep(self.healer_config.start_period).await;
        }

        let mut history_unhealthy = HashMap::<ContainerId, (Option<Box<str>>, usize)>::new();

        let mut interval = tokio::time::interval(self.healer_config.interval);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            interval.tick().await;

            match with_timeout(
                self.docker_timeout,
                self.client.list_containers(&self.filters),
            )
            .await
            {
                Ok(containers) => {
                    let mut new_history =
                        HashMap::<ContainerId, (Option<Box<str>>, usize)>::with_capacity(
                            containers.len(),
                        );

                    for container in containers {
                        let times = history_unhealthy
                            .get(&container.id)
                            .map_or(1, |&(_, t)| t + 1);

                        if container
                            .names
                            .iter()
                            .any(|n| self.healer_config.exclude_containers.contains(n))
                        {
                            event!(
                                Level::INFO,
                                container_name = %container
                                    .get_name()
                                    .unwrap_or("<UNNAMED CONTAINER>"),
                                container_short_id = %container.get_short_id(),
                                "Container is unhealthy, but it is excluded",
                            );
                        } else {
                            self.check_container_health(&container, times).await;
                        }

                        let name = container.get_name().map(Into::into);

                        new_history.insert(container.id, (name, times));
                    }

                    for (key, (name, _)) in history_unhealthy {
                        if !new_history.contains_key(&key) {
                            event!(
                                Level::INFO,
                                container_name = %name.as_deref().unwrap_or("<UNNAMED CONTAINER>"),
                                container_short_id = %key.as_short(),
                                "Container returned to healthy state.",
                            );
                        }
                    }

                    history_unhealthy = new_history;
                },
                Err(error) => {
                    event!(Level::ERROR, ?error, "Failed to fetch container info");
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use hashbrown::HashMap;
    use pretty_assertions::assert_eq;

    use super::get_timeout;

    fn labels(timeout: &str) -> HashMap<Box<str>, Box<str>> {
        HashMap::from_iter([("autoheal.stop.timeout".into(), timeout.into())])
    }

    #[test]
    fn stop_timeout_label_with_units() {
        assert_eq!(
            get_timeout(&labels("1m 30s")),
            Some(Duration::from_secs(90))
        );
    }

    #[test]
    fn stop_timeout_label_without_unit_is_ignored() {
        assert_eq!(get_timeout(&labels("90")), None);
    }

    #[test]
    fn stop_timeout_label_absent() {
        assert_eq!(get_timeout(&HashMap::new()), None);
    }
}
