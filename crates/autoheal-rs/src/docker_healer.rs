use std::time::Duration;

use color_eyre::eyre;
use hashbrown::HashMap;
use reqwest::StatusCode;
use tokio::time::{MissedTickBehavior, sleep, timeout};
use tracing::{Level, event};
use twistlock::client::Client;
use twistlock::endpoint::ApiEndpointCallError;
use twistlock::filters::{Filters, Health, Status};
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

enum Watch {
    /// Consecutive unhealthy polls.
    Unhealthy(usize),
    Recovering,
}

/// The outcome for a container that is no longer in the unhealthy list.
enum Outcome {
    Healthy,
    NoHealthCheck,
    NotRunning(Status),
    Removed,
}

impl Outcome {
    /// `None` while the container is still recovering.
    fn from_state(status: Status, health: Option<&Health>) -> Option<Self> {
        match status {
            Status::Running => match health {
                Some(&Health::Healthy) => Some(Outcome::Healthy),
                Some(&(Health::Starting | Health::Unhealthy)) => None,
                Some(&Health::None) | None => Some(Outcome::NoHealthCheck),
            },
            Status::Restarting => None,
            status @ (Status::Created
            | Status::Removing
            | Status::Paused
            | Status::Exited
            | Status::Dead) => Some(Outcome::NotRunning(status)),
        }
    }

    fn log(&self, container_name: &str, container_short_id: &str) {
        match *self {
            Outcome::Healthy => {
                event!(
                    Level::INFO,
                    %container_name,
                    %container_short_id,
                    "Container returned to healthy state.",
                );
            },
            Outcome::NoHealthCheck => {
                event!(
                    Level::INFO,
                    %container_name,
                    %container_short_id,
                    "Container no longer has a health check.",
                );
            },
            Outcome::NotRunning(ref status) => {
                event!(
                    Level::WARN,
                    %container_name,
                    %container_short_id,
                    %status,
                    "Container is not running.",
                );
            },
            Outcome::Removed => {
                event!(
                    Level::INFO,
                    %container_name,
                    %container_short_id,
                    "Container was removed.",
                );
            },
        }
    }
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

    async fn outcome(&self, container_id: &ContainerId) -> Result<Option<Outcome>, eyre::Report> {
        with_timeout(self.docker_timeout, async {
            match self.client.inspect_container(container_id).await {
                Ok(container) => Ok(Outcome::from_state(
                    container.state.status,
                    container.state.health.as_ref().map(|health| &health.status),
                )),
                Err(ApiEndpointCallError::Daemon { status, .. })
                    if status == StatusCode::NOT_FOUND =>
                {
                    Ok(Some(Outcome::Removed))
                },
                Err(error) => Err(error),
            }
        })
        .await
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

        let mut watched = HashMap::<ContainerId, (Option<Box<str>>, Watch)>::new();

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
                    let mut new_watched =
                        HashMap::<ContainerId, (Option<Box<str>>, Watch)>::with_capacity(
                            containers.len(),
                        );

                    for container in containers {
                        let times = match watched.get(&container.id) {
                            Some(&(_, Watch::Unhealthy(times))) => times + 1,
                            Some(&(_, Watch::Recovering)) | None => 1,
                        };

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

                        new_watched.insert(container.id, (name, Watch::Unhealthy(times)));
                    }

                    for (container_id, (name, _)) in watched {
                        if new_watched.contains_key(&container_id) {
                            continue;
                        }

                        let container_name = name.as_deref().unwrap_or("<UNNAMED CONTAINER>");
                        let container_short_id = container_id.as_short();

                        match self.outcome(&container_id).await {
                            Ok(None) => {
                                new_watched.insert(container_id, (name, Watch::Recovering));
                            },
                            Ok(Some(outcome)) => {
                                outcome.log(container_name, container_short_id);
                            },
                            Err(error) => {
                                event!(
                                    Level::WARN,
                                    ?error,
                                    %container_name,
                                    %container_short_id,
                                    "Failed to inspect container. No longer watching it.",
                                );
                            },
                        }
                    }

                    watched = new_watched;
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
    use twistlock::filters::{Health, Status};

    use super::{Outcome, get_timeout};

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

    #[test]
    fn running_and_healthy_has_recovered() {
        assert!(matches!(
            Outcome::from_state(Status::Running, Some(&Health::Healthy)),
            Some(Outcome::Healthy)
        ));
    }

    #[test]
    fn running_and_starting_is_pending() {
        assert!(Outcome::from_state(Status::Running, Some(&Health::Starting)).is_none());
    }

    #[test]
    fn running_and_unhealthy_is_pending() {
        assert!(Outcome::from_state(Status::Running, Some(&Health::Unhealthy)).is_none());
    }

    #[test]
    fn restarting_is_pending() {
        assert!(Outcome::from_state(Status::Restarting, Some(&Health::Starting)).is_none());
    }

    #[test]
    fn running_without_health_check() {
        assert!(matches!(
            Outcome::from_state(Status::Running, None),
            Some(Outcome::NoHealthCheck)
        ));
        assert!(matches!(
            Outcome::from_state(Status::Running, Some(&Health::None)),
            Some(Outcome::NoHealthCheck)
        ));
    }

    #[test]
    fn exited_is_not_running_despite_its_health() {
        assert!(matches!(
            Outcome::from_state(Status::Exited, Some(&Health::Unhealthy)),
            Some(Outcome::NotRunning(Status::Exited))
        ));
    }
}
