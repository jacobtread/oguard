use anyhow::{anyhow, Context};
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use futures::{stream::FuturesUnordered, StreamExt};
use log::{debug, error, warn};
use native_dialog::{MessageDialog, MessageType};
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
    process::Command,
    select,
    sync::RwLock,
    task::{spawn_blocking, AbortHandle, JoinSet},
    time::{interval_at, sleep, timeout, MissedTickBehavior},
};
use uuid::Uuid;

use crate::{
    ups::{QueryDeviceBattery, UPSExecutorHandle},
    watcher::{UPSEvent, UPSWatcherHandle},
};

type SharedActiveTasks = Arc<RwLock<Vec<EventPipelineTask>>>;

/// Executor for event pipelines
pub struct EventPipelineRunner {
    /// Executor handle for accessing the UPS
    executor: UPSExecutorHandle,
    /// Pipelines to execute
    pipelines: Vec<EventPipeline>,
    /// Watcher handle for events
    watcher_handle: UPSWatcherHandle,
    /// Running task set
    active_tasks: SharedActiveTasks,
    /// Task join set
    join_set: JoinSet<()>,
}

/// Task currently running on the event pipeline runner
struct EventPipelineTask {
    /// Unique ID for the event
    id: Uuid,
    /// abort handle for the task
    abort_handle: AbortHandle,
}

impl EventPipelineRunner {
    /// Creates a new event pipeline runner
    pub fn new(
        executor: UPSExecutorHandle,
        pipelines: Vec<EventPipeline>,
        watcher_handle: UPSWatcherHandle,
    ) -> Self {
        Self {
            executor,
            pipelines,
            watcher_handle,
            active_tasks: Default::default(),
            join_set: Default::default(),
        }
    }

    /// Runs the event pipelines
    pub async fn run(mut self) {
        while let Some(event) = self.watcher_handle.next().await {
            debug!("handling {event} event pipeline");

            // Cancel pipelines that can be cancelled
            self.cancel_pipelines(&event).await;

            // Run the event pipeline
            self.run_pipeline(event).await;
        }
    }

    pub async fn cancel_pipelines(&mut self, event: &UPSEvent) {
        let cancels = event.cancels();

        // Event cancels no other
        if cancels.is_empty() {
            return;
        }

        // Find pipelines this event cancels
        let cancels_pipelines: Vec<&EventPipeline> = self
            .pipelines
            .iter()
            .filter(|pipeline| cancels.contains(&pipeline.event) && pipeline.cancellable)
            .collect();

        // Nothing to cancel
        if cancels_pipelines.is_empty() {
            return;
        }

        debug!(
            "cancelling {} event pipelines for {event}",
            cancels_pipelines.len()
        );

        // Cancel running pipelines that this event should cancel
        self.active_tasks
            .write()
            .await
            // Find matching pipeline arc pointers to tasks
            .retain(|task| {
                let is_cancel = cancels_pipelines
                    .iter()
                    .any(|cancel_pipeline| cancel_pipeline.id == task.id);

                if is_cancel {
                    task.abort_handle.abort();
                }

                !is_cancel
            });
    }

    /// Checks if theres currently an active task for the provided pipeline
    pub async fn is_running_task(&self, pipeline: &EventPipeline) -> bool {
        self.active_tasks
            .read()
            .await
            .iter()
            .any(|task| pipeline.id == task.id)
    }

    pub async fn run_pipeline(&mut self, event: UPSEvent) {
        // Find the pipeline for this event
        let pipeline = self
            .pipelines
            .iter()
            .find(|pipeline| pipeline.event == event);

        let Some(pipeline) = pipeline else {
            // Event has no pipeline to process, continue to next event
            debug!("skipping {event} event with no pipeline handler");
            return;
        };

        if self.is_running_task(pipeline).await {
            // Task is already running
            debug!("skipping event with already running task");
            return;
        }

        // Spawn the task runner
        let abort_handle = self.join_set.spawn(run_pipeline(
            pipeline.clone(),
            self.executor.clone(),
            self.active_tasks.clone(),
        ));

        let id = pipeline.id;

        // Add to the active tasks
        self.active_tasks
            .write()
            .await
            .push(EventPipelineTask { id, abort_handle });
    }
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

/// Executes an action pipeline (Serial)
async fn run_action_pipeline(
    pipeline: ActionPipeline,
    event: UPSEvent,
    executor: UPSExecutorHandle,
) {
    for action in pipeline.actions {
        action.schedule_action(event, &executor).await;
    }
}

/// Runs an event pipeline (Parallel)
async fn run_pipeline(
    pipeline: EventPipeline,
    executor: UPSExecutorHandle,
    active_tasks: SharedActiveTasks,
) {
    let event = pipeline.event;

    debug!("starting {event} task pipeline");

    // Spawn and run the action pipelines
    let mut pipeline_set: FuturesUnordered<_> = pipeline
        .pipelines
        .into_iter()
        .map(|pipeline| run_action_pipeline(pipeline, event, executor.clone()))
        .collect();

    while pipeline_set.next().await.is_some() {}

    debug!("{event} pipeline complete");

    // Remove the completed task
    active_tasks
        .write()
        .await
        .retain(|task| pipeline.id != task.id);
}

/// Pipeline of actions to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPipeline {
    /// Actions this pipeline will execute
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
            ty: ActionType::Shutdown(ShutdownAction {
                force_close_apps: false,
                message: None,
                timeout: None,
            }),
            delay: ActionDelay {
                below_capacity: Some(30),
                duration: Some(Duration::from_secs(60 * 30)),
            },
            repeat: None,
            retry: None,
        }
    }
}

/// Awaits the provided action delay
pub async fn await_action_delay(delay: &ActionDelay, executor: &UPSExecutorHandle) {
    // Await the required delay
    match (delay.duration, delay.below_capacity) {
        // Run below capacity
        (None, Some(capacity)) => {
            await_capacity(executor, capacity).await;
        }
        // Run after fixed duration
        (Some(duration), None) => {
            sleep(duration).await;
        }
        // Run after fixed duration or below capacity
        (Some(duration), Some(capacity)) => {
            select! {
                _ = await_capacity(executor, capacity) => {}
                _ = sleep(duration) => {}
            };
        }
        // Run immediately
        (None, None) => {}
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

/// Awaits the delay before repeating an action
pub async fn await_repeat_delay(repeat: &ActionRepeat, executor: &UPSExecutorHandle) {
    match (repeat.interval, repeat.capacity_decrease) {
        // Run when capacity decreases by amount
        (None, Some(capacity)) => {
            await_capacity_decrease(executor, capacity).await;
        }
        // Run after fixed duration
        (Some(duration), None) => {
            sleep(duration).await;
        }
        // Run after fixed duration or below capacity
        (Some(duration), Some(capacity)) => {
            select! {
                _ = await_capacity_decrease(executor, capacity) => {}
                _ = sleep(duration) => {}
            };
        }
        // Don't repeat
        (None, None) => {}
    }
}

/// Polls the provided executor until the capacity decreases by the provided amount
pub async fn await_capacity_decrease(executor: &UPSExecutorHandle, decrease: u8) {
    let mut last_highest_capacity: Option<u8> = None;
    let mut last_lowest_capacity: Option<u8> = None;

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

        // Update the highest capacity
        let highest_capacity = match last_highest_capacity.as_ref() {
            Some(&value) => {
                if battery_state.capacity > value {
                    *last_highest_capacity.insert(battery_state.capacity)
                } else {
                    value
                }
            }
            None => *last_highest_capacity.insert(battery_state.capacity),
        };

        // Update the lowest capacity
        let lowest_capacity = match last_lowest_capacity.as_ref() {
            Some(&value) => {
                if battery_state.capacity < value {
                    *last_lowest_capacity.insert(battery_state.capacity)
                } else {
                    value
                }
            }
            None => *last_lowest_capacity.insert(battery_state.capacity),
        };

        let difference = highest_capacity.saturating_sub(lowest_capacity);

        if difference >= decrease {
            return;
        }

        interval.tick().await;
    }
}

impl Action {
    /// Will run the action asynchronously when the action is ready and handle
    /// waiting for repeated delays
    pub async fn schedule_action(&self, event: UPSEvent, executor: &UPSExecutorHandle) {
        await_action_delay(&self.delay, executor).await;

        let mut execution = 0;

        loop {
            execution += 1;
            self.execute_with_retry(event, executor).await;

            // Handle action repeating
            let Some(repeat) = self.repeat.as_ref() else {
                break;
            };

            let can_repeat = repeat
                .limit
                // Can repeat if our execution count is less than the defined limit
                .map(|value| execution < value)
                // Can always repeat when no limit
                .unwrap_or(true);

            if !can_repeat {
                break;
            }

            debug!("awaiting task repeat delay");

            // Await the repeating delay
            await_repeat_delay(repeat, executor).await;

            debug!("repeating task");
        }
    }

    /// Executes the action and handles retry on failure
    pub async fn execute_with_retry(&self, event: UPSEvent, executor: &UPSExecutorHandle) {
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

    /// Executes the action
    pub async fn execute_action(
        &self,
        event: UPSEvent,
        executor: &UPSExecutorHandle,
    ) -> anyhow::Result<()> {
        match &self.ty {
            ActionType::Notification => execute_notification(event).await,
            ActionType::Popup => execute_popup(event).await,
            ActionType::Sleep => execute_sleep().await,
            ActionType::Shutdown(config) => execute_shutdown(event, config).await,
            ActionType::ShutdownUPS => execute_shutdown_ups(executor).await,
            ActionType::Executable(executable) => execute_executable(executable).await,
            ActionType::HttpRequest(request) => execute_http_request(event, request).await,
        }
    }
}

/// Sends a desktop notification for the provided event
pub async fn execute_notification(event: UPSEvent) -> anyhow::Result<()> {
    let event_name = event.to_string();

    let label_key = format!("event.{}.label", event_name);
    let description_key = format!("event.{}.description", event_name);

    let label = t!(&label_key);
    let description = t!(&description_key);

    let icon = match event {
        UPSEvent::ACFailure => "dialog-negative",
        UPSEvent::UPSFault => "dialog-negative",
        _ => "dialog-positive",
    };

    Notification::new()
        .summary(&label)
        .body(&description)
        .icon(icon)
        .show()
        .context("failed to show notification")?;

    Ok(())
}

/// Shows a popup window for the provided event
pub async fn execute_popup(event: UPSEvent) -> anyhow::Result<()> {
    let event_name = event.to_string();

    let ty = match event {
        UPSEvent::ACFailure => MessageType::Error,
        UPSEvent::UPSFault => MessageType::Error,
        _ => MessageType::Info,
    };

    // Message dialogs are blocking until user action so they're moved to a new thread
    std::thread::spawn(move || {
        let label_key = format!("event.{}.label", event_name);
        let description_key = format!("event.{}.description", event_name);

        let label = t!(&label_key);
        let description = t!(&description_key);

        if let Err(err) = MessageDialog::new()
            .set_title(&label)
            .set_text(&description)
            .set_type(ty)
            .show_alert()
        {
            error!("failed to show popup for {event}: {err}");
        }
    });

    Ok(())
}

/// Puts the system to sleep
pub async fn execute_sleep() -> anyhow::Result<()> {
    spawn_blocking(system_shutdown::sleep)
        .await
        .context("failed to join sleep task")?
        .context("failed to sleep")?;

    Ok(())
}

/// Executes a shutdown
pub async fn execute_shutdown(event: UPSEvent, config: &ShutdownAction) -> anyhow::Result<()> {
    let message = config
        .message
        .as_ref()
        .map(|value| value.to_string())
        .unwrap_or_else(|| format!("Shutdown triggered by {event} pipeline"));
    let timeout = config.timeout.unwrap_or(u32::MAX);
    let force_close_apps = config.force_close_apps;

    spawn_blocking(move || {
        system_shutdown::shutdown_with_message(&message, timeout, force_close_apps)
    })
    .await
    .context("failed to join shutdown task")?
    .context("failed to shutdown")?;

    Ok(())
}

/// Triggers the UPS to shutdown
pub async fn execute_shutdown_ups(executor: &UPSExecutorHandle) -> anyhow::Result<()> {
    Ok(())
}

/// Starts an executable process
pub async fn execute_executable(executable: &ExecutableAction) -> anyhow::Result<()> {
    let child = Command::new(&executable.exe)
        .args(&executable.args)
        .kill_on_drop(true)
        .spawn()
        .context("failed to start executable")?;

    let output = match executable.timeout {
        Some(duration) => match timeout(duration, child.wait_with_output()).await {
            Err(_) => {
                warn!("executable task timed out");
                return Ok(());
            }
            Ok(value) => value,
        },
        None => child.wait_with_output().await,
    }
    .context("error getting executable output")?;

    let status = output.status;
    if status.success() {
        return Ok(());
    }

    let message = String::from_utf8(output.stderr).unwrap_or_default();
    let message = format!("executable non zero exit code: {message}");

    Err(anyhow!(message))
}

/// Sends an HTTP request
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

    if let Some(timeout) = request.timeout.as_ref() {
        builder = builder.timeout(*timeout);
    }

    if let Some(body) = request.body.as_ref() {
        builder = builder.body(body.to_string());
    }

    let request = builder.build().context("building http request")?;

    let _response = client
        .execute(request)
        .await
        .context("error sending request")?
        .error_for_status()
        .context("response error")?;

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
    Shutdown(ShutdownAction),

    /// Shutdown the UPS itself
    ShutdownUPS,

    /// Run an executable
    Executable(ExecutableAction),

    /// Send an HTTP request
    HttpRequest(HttpRequestAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownAction {
    /// Optional message to show
    message: Option<String>,

    /// Timeout for shutdown force close
    timeout: Option<u32>,

    /// Whether to force close apps
    force_close_apps: bool,
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
    /// Optional request timeout
    timeout: Option<Duration>,
}

/// Configuration for the delay of an actions execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDelay {
    /// Run the action after a fix duration
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

    /// Maximum number of times to repeat
    pub limit: Option<u8>,
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
        action::ExecutableAction,
        ups::UPSExecutor,
        watcher::{UPSEvent, UPSWatcherHandle},
    };

    use super::{
        Action, ActionDelay, ActionPipeline, ActionRepeat, ActionType, EventPipeline,
        EventPipelineRunner,
    };

    #[tokio::test]
    #[ignore]
    async fn test_notification_action() {
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
                    repeat: Some(ActionRepeat {
                        interval: Some(Duration::from_secs(10)),
                        capacity_decrease: None,
                        limit: Some(3),
                    }),
                    retry: None,
                }],
            }],
            cancellable: false,
        }];

        debug!("spawning runner");

        tokio::spawn(EventPipelineRunner::new(executor, pipelines, watcher_handle).run());

        debug!("sending event");

        _ = tx.send(UPSEvent::ACFailure);

        loop {
            sleep(Duration::from_secs(60)).await;
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_popup_action() {
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
                    ty: ActionType::Popup,
                    delay: ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    },
                    repeat: Some(ActionRepeat {
                        interval: Some(Duration::from_secs(10)),
                        capacity_decrease: None,
                        limit: Some(3),
                    }),
                    retry: None,
                }],
            }],
            cancellable: false,
        }];

        debug!("spawning runner");

        tokio::spawn(EventPipelineRunner::new(executor, pipelines, watcher_handle).run());

        debug!("sending event");

        _ = tx.send(UPSEvent::ACFailure);

        loop {
            sleep(Duration::from_secs(60)).await;
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_sleep_action() {
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
                    ty: ActionType::Sleep,
                    delay: ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    },
                    repeat: Some(ActionRepeat {
                        interval: Some(Duration::from_secs(10)),
                        capacity_decrease: None,
                        limit: Some(0),
                    }),
                    retry: None,
                }],
            }],
            cancellable: false,
        }];

        debug!("spawning runner");

        tokio::spawn(EventPipelineRunner::new(executor, pipelines, watcher_handle).run());

        debug!("sending event");

        _ = tx.send(UPSEvent::ACFailure);

        loop {
            sleep(Duration::from_secs(60)).await;
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_executable_action() {
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
                    ty: ActionType::Executable(ExecutableAction {
                        exe: "notepad.exe".to_string(),
                        args: vec![],
                        timeout: None,
                    }),
                    delay: ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    },
                    repeat: Some(ActionRepeat {
                        interval: Some(Duration::from_secs(10)),
                        capacity_decrease: None,
                        limit: Some(0),
                    }),
                    retry: None,
                }],
            }],
            cancellable: false,
        }];

        debug!("spawning runner");

        tokio::spawn(EventPipelineRunner::new(executor, pipelines, watcher_handle).run());

        debug!("sending event");

        _ = tx.send(UPSEvent::ACFailure);

        loop {
            sleep(Duration::from_secs(60)).await;
        }
    }
}
