use anyhow::Context;
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use log::{debug, error};
use notify_rust::Notification;
use reqwest::Method;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    select,
    sync::RwLock,
    task::{AbortHandle, JoinSet},
    time::{interval_at, sleep, MissedTickBehavior},
};
use uuid::Uuid;

use crate::{
    ups::{QueryDeviceBattery, UPSExecutorHandle},
    watcher::{UPSEvent, UPSWatcherHandle},
};

pub struct EventPipelineRunner {
    /// Executor handle for accessing the UPS
    pub executor: UPSExecutorHandle,
    /// Pipelines to execute
    pub pipelines: Vec<EventPipeline>,
    /// Watcher handle for events
    pub watcher_handle: UPSWatcherHandle,
}

impl EventPipelineRunner {
    pub async fn run(mut self) {
        let active_tasks: Arc<RwLock<Vec<EventPipelineTask>>> = Default::default();
        let mut join_set: JoinSet<()> = JoinSet::new();

        while let Some(event) = self.watcher_handle.next().await {
            debug!("handling {event} event pipeline");

            // Find the pipeline for this event
            let pipeline = self
                .pipelines
                .iter()
                .find(|pipeline| pipeline.event == event);

            // Find pipelines this event cancels
            let cancels_pipelines: Vec<EventPipeline> = {
                let cancels = event.cancels();
                self.pipelines
                    .iter()
                    .filter(|pipeline| cancels.contains(&pipeline.event) && pipeline.cancellable)
                    .cloned()
                    .collect()
            };

            if !cancels_pipelines.is_empty() {
                debug!(
                    "cancelling {} event pipelines for {event}",
                    cancels_pipelines.len()
                );
            }

            // Cancel running pipelines that this event should cancel
            {
                active_tasks
                    .write()
                    .await
                    // Find matching pipeline arc pointers to tasks
                    .retain(|task| {
                        let is_cancel = cancels_pipelines
                            .iter()
                            .any(|cancel_pipeline| cancel_pipeline.id == task.id);

                        if is_cancel {
                            task.abort_handle.abort();
                            false
                        } else {
                            true
                        }
                    });
            }

            let Some(pipeline) = pipeline else {
                // Event has no pipeline to process, continue to next event
                debug!("skipping {event} event with no pipeline handler");
                continue;
            };

            let existing_task = {
                active_tasks
                    .read()
                    .await
                    .iter()
                    .any(|task| pipeline.id == task.id)
            };

            if existing_task {
                // Task is already running
                debug!("skipping event with already running task");
                continue;
            }

            let pipeline = pipeline.clone();
            let id = pipeline.id;

            let abort_handle = {
                let executor = self.executor.clone();
                let active_tasks = active_tasks.clone();

                // Spawn the task runner
                join_set.spawn(async move {
                    let executor = executor;
                    let mut joint_set = JoinSet::new();

                    debug!("starting {event} task pipeline");

                    for action_pipeline in pipeline.pipelines {
                        let executor = executor.clone();

                        // Spawn each action pipeline
                        joint_set.spawn(async move {
                            for action in action_pipeline.actions {
                                action.schedule_action(event, &executor).await;
                            }
                        });
                    }

                    // Join the tasks
                    while (joint_set.join_next().await).is_some() {}

                    debug!("{event} pipeline complete");

                    // Remove the completed task
                    active_tasks
                        .write()
                        .await
                        .retain(|task| pipeline.id != task.id);
                })
            };

            // Add to the active tasks
            active_tasks
                .write()
                .await
                .push(EventPipelineTask { id, abort_handle });
        }
    }
}

pub struct EventPipelineTask {
    /// Unique ID for the event
    pub id: Uuid,
    /// abort handle for the task
    pub abort_handle: AbortHandle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPipeline {
    /// Unique ID for the event
    pub id: Uuid,
    /// Event that triggers the pipeline
    pub event: UPSEvent,
    /// Action pipelines to run
    pub pipelines: Vec<ActionPipeline>,
    /// Whether the events that cancel this should abort the run
    pub cancellable: bool,
}

/// Pipeline of actions to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPipeline {
    actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Action type
    pub ty: ActionType,
    /// Delay for the action
    pub delay: ActionDelay,
    /// Optionally repeat
    pub repeat: Option<ActionRepeat>,
    /// Optionally retry
    pub retry: Option<ActionRetry>,
}

impl Default for Action {
    fn default() -> Self {
        Action {
            ty: ActionType::Shutdown,
            delay: ActionDelay {
                below_capacity: Some(30),
                duration: Some(Duration::from_secs(60 * 30)),
            },
            repeat: None,
            retry: None,
        }
    }
}

impl Action {
    pub async fn schedule_action(&self, event: UPSEvent, executor: &UPSExecutorHandle) {
        // Await the required delay
        match (self.delay.duration, self.delay.below_capacity) {
            // Run below capacity
            (None, Some(capacity)) => {
                Self::await_capacity(executor, capacity).await;
            }
            // Run after fixed duration
            (Some(duration), None) => {
                sleep(duration).await;
            }
            // Run after fixed duration or below capacity
            (Some(duration), Some(capacity)) => {
                select! {
                    _ = Self::await_capacity(executor, capacity) => {}
                    _ = sleep(duration) => {}
                };
            }
            // Run immediately
            (None, None) => {}
        }

        let mut attempt = 0;
        let mut last_delay: Option<Duration> = None;

        loop {
            // Try and execute the action
            let Err(err) = self.execute_action(event, executor).await else {
                break;
            };

            error!("error processing action: {err}");

            // Only continue when a retry action is available
            let Some(retry) = self.retry.as_ref() else {
                break;
            };

            // Max attempts reached
            if attempt > retry.max_attempts {
                break;
            }

            attempt += 1;

            match retry.delay {
                ActionRetryDelay::Fixed { delay } => sleep(delay).await,
                ActionRetryDelay::LinearBackoff { initial, increment } => {
                    let current_delay = last_delay
                        .map(|last_delay| last_delay.saturating_add(increment))
                        .unwrap_or(initial);

                    last_delay = Some(current_delay);

                    sleep(current_delay).await;
                }
                ActionRetryDelay::ExponentialBackoff { initial, exponent } => {
                    let current_delay = last_delay
                        .map(|last_delay| last_delay.saturating_mul(exponent))
                        .unwrap_or(initial);

                    last_delay = Some(current_delay);

                    sleep(current_delay).await;
                }
            }
        }
    }

    /// Future that polls the device capacity resolving when the capacity gets
    /// below the provided threshold
    pub async fn await_capacity(executor: &UPSExecutorHandle, capacity: u8) {
        let poll_interval = Duration::from_secs(1);

        let start = Instant::now() + poll_interval;
        let mut interval = interval_at(start.into(), poll_interval);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        // Loop until capacity reaches threshold
        loop {
            let battery_state = match executor.request(QueryDeviceBattery).await {
                Ok(value) => value,
                Err(err) => {
                    error!("Error while requesting UPS device battery: {err:?}");
                    continue;
                }
            };

            if battery_state.capacity < capacity {
                return;
            }

            interval.tick().await;
        }
    }

    pub async fn execute_action(
        &self,
        event: UPSEvent,
        executor: &UPSExecutorHandle,
    ) -> anyhow::Result<()> {
        match &self.ty {
            ActionType::Notification => execute_notification(event).await,
            ActionType::Popup => execute_popup(event).await,
            ActionType::Sleep => execute_sleep(event).await,
            ActionType::Shutdown => execute_shutdown(event).await,
            ActionType::ShutdownUPS => execute_shutdown_ups(event, executor).await,
            ActionType::Executable(executable) => execute_executable(event, executable).await,
            ActionType::HttpRequest(request) => execute_http_request(event, request).await,
        }
    }
}

pub async fn execute_notification(event: UPSEvent) -> anyhow::Result<()> {
    let event_name = event.to_string();

    let label_key = format!("event.{}.label", event_name);
    let description_key = format!("event.{}.description", event_name);

    let label = t!(&label_key);
    let description = t!(&description_key);

    let icon = match event {
        UPSEvent::ACFailure => "dialog-negative",
        UPSEvent::ACRecovery => "dialog-positive",
        UPSEvent::UPSFault => "dialog-negative",
        _ => "dialog-positive",
    };

    Notification::new()
        .summary(&label)
        .body(&description)
        .icon(icon)
        .show()?;

    Ok(())
}

pub async fn execute_popup(event: UPSEvent) -> anyhow::Result<()> {
    Ok(())
}

pub async fn execute_sleep(event: UPSEvent) -> anyhow::Result<()> {
    Ok(())
}

pub async fn execute_shutdown(event: UPSEvent) -> anyhow::Result<()> {
    Ok(())
}

pub async fn execute_shutdown_ups(
    event: UPSEvent,
    executor: &UPSExecutorHandle,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn execute_executable(
    event: UPSEvent,
    executable: &ExecutableAction,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn execute_http_request(
    event: UPSEvent,
    request: &HttpRequestAction,
) -> anyhow::Result<()> {
    let method = Method::from_str(&request.method).context("invalid http method")?;
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    for (key, value) in &request.headers {
        let key = HeaderName::from_str(key).context("invalid header key")?;
        let value = HeaderValue::from_str(value).context("invalid header key")?;

        headers.insert(key, value);
    }

    let mut builder = client.request(method, &request.url).headers(headers);

    if let Some(body) = request.body.as_ref() {
        builder = builder.body(body.to_string());
    }

    let request = builder.build().context("building http request")?;

    let response = client
        .execute(request)
        .await
        .context("error sending request")?;

    // TODO: Handle error

    Ok(())
}

/// Actions the task executor can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Send desktop notification
    Notification,

    /// Show popup notification
    Popup,

    /// Put the device to sleep
    Sleep,

    /// Shutdown the device
    Shutdown,

    /// Shutdown the UPS itself
    ShutdownUPS,

    /// Run an executable
    Executable(ExecutableAction),

    /// Send an HTTP request
    HttpRequest(HttpRequestAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutableAction {
    /// Executable to run
    exe: String,

    /// Arguments for the program
    args: Vec<String>,

    /// Timeout for the program run
    timeout: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestAction {
    /// URL to send the request to
    url: String,
    /// HTTP method to use
    method: String,
    /// Headers to put on the request
    headers: HashMap<String, String>,
    /// Optional request body
    body: Option<String>,
}

/// Configuration for the delay of an actions execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDelay {
    /// Run the action
    pub duration: Option<Duration>,

    /// Run immediately if the capacity is less that or equal to this amount
    /// overrides the duration delay
    pub below_capacity: Option<u8>,
}

/// Configuration for how an action should repeat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRepeat {
    /// Run at a fixed interval
    pub interval: Option<Duration>,

    /// Every time the capacity decreases by minimum this amount
    pub capacity_decrease: Option<u8>,
}

/// Configuration for how an action should retry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRetry {
    /// Mode for the retry delay
    pub delay: ActionRetryDelay,
    /// Maximum number of retry attempts
    pub max_attempts: u8,
}

/// Options for how a retry delay should be determined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionRetryDelay {
    /// Retry at fixed intervals
    Fixed { delay: Duration },
    /// Retry at a linearly increasing rate (5s, 10s, 15s, 20s)
    LinearBackoff {
        /// Initial backoff duration
        initial: Duration,
        /// Amount to add per failed attempt
        increment: Duration,
    },
    // Retry at a exponentially increasing rate (5s, 10s, 20s, 40s)
    ExponentialBackoff {
        /// Initial backoff duration
        initial: Duration,
        /// Exponent to multiply by
        exponent: u32,
    },
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use log::debug;
    use tokio::{sync::broadcast, time::sleep};
    use uuid::Uuid;

    use crate::{
        ups::UPSExecutor,
        watcher::{UPSEvent, UPSWatcherHandle},
    };

    use super::{
        Action, ActionDelay, ActionPipeline, ActionType, EventPipeline, EventPipelineRunner,
    };

    #[tokio::test]
    async fn test_action() {
        dotenvy::dotenv().unwrap();
        env_logger::init();
        log_panics::init();
        let (tx, rx) = broadcast::channel(8);
        let watcher_handle = UPSWatcherHandle { rx };
        let executor = UPSExecutor::start().unwrap();
        let pipelines = vec![EventPipeline {
            id: Uuid::new_v4(),
            event: UPSEvent::ACFailure,
            pipelines: vec![ActionPipeline {
                actions: vec![Action {
                    ty: ActionType::Notification,
                    delay: ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    },
                    repeat: None,
                    retry: None,
                }],
            }],
            cancellable: false,
        }];

        debug!("spawning runner");

        tokio::spawn(
            EventPipelineRunner {
                executor,
                pipelines,
                watcher_handle,
            }
            .run(),
        );

        debug!("sending event");

        _ = tx.send(UPSEvent::ACFailure);

        loop {
            sleep(Duration::from_secs(60)).await;
        }
    }
}
