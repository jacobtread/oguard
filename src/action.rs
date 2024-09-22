use crate::{
    database::entities::{
        event_pipeline::{CancellableEventPipeline, EventPipelineId, EventPipelineModel},
        events::UPSEvent,
    },
    services::watcher::UPSWatcherHandle,
    ups::{device::Device, DeviceExecutorHandle, QueryDeviceBattery, ScheduleUPSShutdown},
    utils::validate::is_non_zero_duration,
};
use anyhow::{anyhow, Context};
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use chrono::Utc;
use futures::{stream::FuturesUnordered, StreamExt};
use garde::Validate;
use log::{debug, error, warn};
use native_dialog::{MessageDialog, MessageType};
use notify_rust::Notification;
use ordered_float::OrderedFloat;
use reqwest::{header, Method};
use rust_i18n::t;
use sea_orm::{DatabaseConnection, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
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

type SharedActiveTasks = Arc<RwLock<Vec<EventPipelineTask>>>;

/// Executor for event pipelines
pub struct EventPipelineRunner {
    /// Executor handle for accessing the UPS
    executor: DeviceExecutorHandle,
    /// Database to load the event pipelines from
    db: DatabaseConnection,
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
    id: EventPipelineId,
    /// abort handle for the task
    abort_handle: AbortHandle,
}

impl EventPipelineRunner {
    /// Creates a new event pipeline runner
    pub fn new(
        db: DatabaseConnection,
        watcher_handle: UPSWatcherHandle,
        executor: DeviceExecutorHandle,
    ) -> Self {
        Self {
            executor,
            db,
            watcher_handle,
            active_tasks: Default::default(),
            join_set: Default::default(),
        }
    }

    /// Starts a new event pipeline runner from the provided parts. Listens
    /// using the provided `watcher_handle` loading pipelines from the provided
    /// `db` sending UPS requests to the provided `executor`
    ///
    /// This will run as a background task
    pub fn start(
        db: DatabaseConnection,
        watcher_handle: UPSWatcherHandle,
        executor: DeviceExecutorHandle,
    ) {
        let runner = Self::new(db, watcher_handle, executor);
        tokio::spawn(runner.run());
    }

    /// Runs the event pipelines
    pub async fn run(mut self) {
        while let Some(event) = self.watcher_handle.next().await {
            debug!("handling {event} event pipeline");

            // Cancel pipelines that can be cancelled
            self.cancel_pipelines(&event).await;

            // Find pipelines to run
            let pipelines = match EventPipelineModel::find_by_event_enabled(&self.db, event).await {
                Ok(value) => value,
                Err(err) => {
                    error!("failed to query event pipelines for event {event}: {err}");
                    continue;
                }
            };

            if pipelines.is_empty() {
                // Event has no pipelines to process, continue to next event
                debug!("skipping {event} event with no pipeline handler");
                continue;
            }

            for pipeline in pipelines {
                // Start the event pipeline
                self.start_pipeline(event, pipeline).await;
            }
        }
    }

    pub async fn cancel_pipelines(&mut self, event: &UPSEvent) {
        let cancels = event.cancels();

        // Event cancels no other
        if cancels.is_empty() {
            debug!("no events to cancel");
            return;
        }

        // Find pipelines this event cancels
        let cancels_pipelines: Vec<CancellableEventPipeline> =
            match EventPipelineModel::find_cancellable(&self.db, cancels.to_vec()).await {
                Ok(value) => value,
                Err(err) => {
                    error!("failed to query cancellable event pipelines for {event}: {err}");
                    return;
                }
            };

        // Nothing to cancel
        if cancels_pipelines.is_empty() {
            debug!("no pipelines to cancel");
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
                    debug!("aborting running task: {}", task.id);
                    task.abort_handle.abort();
                }

                !is_cancel
            });
    }

    /// Checks if theres currently an active task for the provided pipeline
    pub async fn is_running_task(&self, id: EventPipelineId) -> bool {
        self.active_tasks
            .read()
            .await
            .iter()
            .any(|task| id == task.id)
    }

    pub async fn start_pipeline(&mut self, event: UPSEvent, pipeline: EventPipelineModel) {
        let id = pipeline.id;

        if self.is_running_task(id).await {
            // Task is already running
            debug!("skipping event with already running task");
            return;
        }

        // Spawn the task runner
        let abort_handle = self.join_set.spawn(run_pipeline(
            self.db.clone(),
            pipeline,
            self.executor.clone(),
            self.active_tasks.clone(),
            event,
        ));

        // Add to the active tasks
        self.active_tasks
            .write()
            .await
            .push(EventPipelineTask { id, abort_handle });
    }
}

/// Runs an event pipeline (Parallel)
async fn run_pipeline(
    db: DatabaseConnection,
    pipeline: EventPipelineModel,
    executor: DeviceExecutorHandle,
    active_tasks: SharedActiveTasks,
    event: UPSEvent,
) {
    let name = &pipeline.name;

    debug!("starting \"{name}\" ({event}) task pipeline");

    let mut repeated = Vec::new();

    for action in pipeline.pipeline.actions {
        // Attempt to run the action
        if !action.schedule_action(event, &executor).await {
            return;
        }

        // Queue action repeats
        if action.repeat.is_some() {
            repeated.push(action)
        }
    }

    // Update time of last execution
    if let Err(err) = EventPipelineModel::set_last_executed(&db, pipeline.id, Utc::now()).await {
        error!(
            "failed to update last executed timestamp for {} ({}): {}",
            pipeline.name, pipeline.id, err
        );
    }

    // Futures that can be repeated are handled out of order
    let mut repeated_futures: FuturesUnordered<_> = repeated
        .into_iter()
        .map(|action| run_repeated_action(action, event, executor.clone()))
        .collect();

    while repeated_futures.next().await.is_some() {}

    debug!("\"{name}\" ({event})  pipeline complete");

    // Remove the completed task
    active_tasks
        .write()
        .await
        .retain(|task| pipeline.id != task.id);
}

/// Runs an event pipeline ignoring any delays and without
/// running repeated tasks
pub async fn run_pipeline_test<D: Device>(
    pipeline: EventPipelineModel,
    executor: DeviceExecutorHandle<D>,
    event: UPSEvent,
) {
    let name = &pipeline.name;

    debug!("starting \"{name}\" ({event}) task pipeline test");

    for action in pipeline.pipeline.actions {
        // Attempt to run the action
        if !action.execute_with_retry(event, &executor).await {
            return;
        }
    }

    debug!("\"{name}\" ({event}) pipeline test complete");
}

/// Executes the repeated portion of an action
async fn run_repeated_action(action: Action, event: UPSEvent, executor: DeviceExecutorHandle) {
    let Some(repeat) = action.repeat.as_ref() else {
        panic!("attempted to run non repeating action as repeat action")
    };

    let mut execution = 0;

    while action.execute_with_retry(event, &executor).await {
        execution += 1;

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
        await_repeat_delay(repeat, &executor).await;

        debug!("repeating task");
    }
}

/// Pipeline of actions to execute
#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ActionPipeline {
    /// Actions this pipeline will execute
    #[garde(dive)]
    actions: Vec<Action>,
}

#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    /// Action type
    #[garde(dive)]
    pub ty: ActionType,
    /// Delay for the action
    #[garde(dive)]
    pub delay: Option<ActionDelay>,
    /// Optionally repeat, initial action run will be in the defined action pipeline
    /// order any additional repeat runs will happen out of order
    #[garde(dive)]
    pub repeat: Option<ActionRepeat>,
    /// Optionally retry
    #[garde(dive)]
    pub retry: Option<ActionRetry>,
}

/// Awaits the provided action delay
pub async fn await_action_delay(delay: &ActionDelay, executor: &DeviceExecutorHandle) {
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
pub async fn await_capacity(executor: &DeviceExecutorHandle, capacity: u8) {
    let poll_interval = Duration::from_secs(1);

    let start = Instant::now() + poll_interval;
    let mut interval = interval_at(start.into(), poll_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    // Loop until capacity reaches threshold
    loop {
        let battery_state = match executor.send(QueryDeviceBattery).await {
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
pub async fn await_repeat_delay(repeat: &ActionRepeat, executor: &DeviceExecutorHandle) {
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
pub async fn await_capacity_decrease(executor: &DeviceExecutorHandle, decrease: u8) {
    let mut last_highest_capacity: Option<u8> = None;
    let mut last_lowest_capacity: Option<u8> = None;

    let poll_interval = Duration::from_secs(1);

    let start = Instant::now() + poll_interval;
    let mut interval = interval_at(start.into(), poll_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    // Loop until capacity reaches threshold
    loop {
        let battery_state = match executor.send(QueryDeviceBattery).await {
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
    pub async fn schedule_action(&self, event: UPSEvent, executor: &DeviceExecutorHandle) -> bool {
        if let Some(delay) = self.delay.as_ref() {
            await_action_delay(delay, executor).await;
        }

        self.execute_with_retry(event, executor).await
    }

    /// Executes the action and handles retry on failure
    pub async fn execute_with_retry<D: Device>(
        &self,
        event: UPSEvent,
        executor: &DeviceExecutorHandle<D>,
    ) -> bool {
        let mut attempt = 0;
        let mut last_delay: Option<Duration> = None;

        loop {
            // Try and execute the action
            let Err(err) = self.execute_action(event, executor).await else {
                return true;
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
                        .map(|last_delay| last_delay.saturating_mul(exponent as u32))
                        .unwrap_or(initial);

                    last_delay = Some(current_delay);

                    sleep(current_delay).await;
                }
            }
        }

        false
    }

    /// Executes the action
    pub async fn execute_action<D: Device>(
        &self,
        event: UPSEvent,
        executor: &DeviceExecutorHandle<D>,
    ) -> anyhow::Result<()> {
        match &self.ty {
            ActionType::Notification => execute_notification(event).await,
            ActionType::Popup => execute_popup(event).await,
            ActionType::Sleep => execute_sleep().await,
            ActionType::Shutdown(config) => execute_shutdown(event, config).await,
            ActionType::USPShutdown(config) => execute_shutdown_ups(config, executor).await,
            ActionType::Executable(executable) => execute_executable(event, executable).await,
            ActionType::HttpRequest(request) => execute_http_request(event, request).await,
        }
    }
}

/// Actions the task executor can perform
#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionType {
    /// Send desktop notification
    Notification,

    /// Show popup notification
    Popup,

    /// Put the device to sleep
    Sleep,

    /// Shutdown the device
    Shutdown(#[garde(dive)] ShutdownAction),

    /// Shutdown the UPS itself
    USPShutdown(#[garde(dive)] UPSShutdownAction),

    /// Run an executable
    Executable(#[garde(dive)] ExecutableAction),

    /// Send an HTTP request
    HttpRequest(#[garde(dive)] HttpRequestAction),
}

#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShutdownAction {
    /// Optional message to show
    #[garde(inner(length(min = 1)))]
    message: Option<String>,

    /// Timeout before shutdown,
    ///
    /// WARNING: Should cap this duration to the max windows
    /// shutdown duration, otherwise shutdown will error
    #[garde(inner(custom(is_non_zero_duration)))]
    timeout: Option<Duration>,

    /// Whether to force close apps
    #[garde(skip)]
    force_close_apps: bool,
}

#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UPSShutdownAction {
    /// Delay in minutes before shutting down the UPS
    #[garde(skip)]
    delay_minutes: OrderedFloat<f32>,
}

#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutableAction {
    /// Executable to run
    #[garde(length(min = 1))]
    exe: String,

    /// Arguments for the program
    #[garde(inner(length(min = 1)))]
    args: Vec<String>,

    /// Timeout for the program run
    #[garde(inner(custom(is_non_zero_duration)))]
    timeout: Option<Duration>,
}

#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpRequestAction {
    /// URL to send the request to
    #[garde(url)]
    url: String,
    /// HTTP method to use
    #[garde(skip)]
    method: String,
    /// Headers to put on the request
    #[garde(skip)]
    headers: HashMap<String, String>,
    /// Optional request body
    #[garde(skip)]
    body: Option<HttpRequestActionBody>,
    /// Optional request timeout
    #[garde(inner(custom(is_non_zero_duration)))]
    timeout: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpRequestActionBody {
    /// Payload to send, supports placeholders
    payload: String,
    /// Content type header value to use
    content_type: String,
}

/// Serializer for the pre-defined http request JSON body
#[derive(Serialize)]
pub struct HttpRequestJsonBody {
    /// The event
    pub event: UPSEvent,
}

/// Configuration for the delay of an actions execution
#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionDelay {
    /// Run the action after a fix duration
    #[garde(inner(custom(is_non_zero_duration)))]
    pub duration: Option<Duration>,

    /// Run immediately if the capacity is less that or equal to this amount
    /// overrides the duration delay
    #[garde(range(min = 1, max = 100))]
    pub below_capacity: Option<u8>,
}

/// Configuration for how an action should repeat
#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRepeat {
    /// Run at a fixed interval
    #[garde(inner(custom(is_non_zero_duration)))]
    pub interval: Option<Duration>,

    /// Every time the capacity decreases by minimum this amount
    #[garde(range(min = 1, max = 100))]
    pub capacity_decrease: Option<u8>,

    /// Maximum number of times to repeat
    #[garde(range(min = 0, max = 255))]
    pub limit: Option<u8>,
}

/// Configuration for how an action should retry
#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRetry {
    /// Mode for the retry delay
    #[garde(dive)]
    pub delay: ActionRetryDelay,
    /// Maximum number of retry attempts
    #[garde(range(min = 1))]
    pub max_attempts: u8,
}

/// Options for how a retry delay should be determined
#[derive(Debug, Validate, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionRetryDelay {
    /// Retry at fixed intervals
    Fixed {
        /// Delay duration, must be greater than zero
        #[garde(custom(is_non_zero_duration))]
        delay: Duration,
    },
    /// Retry at a linearly increasing rate (5s, 10s, 15s, 20s)
    LinearBackoff {
        /// Initial backoff duration
        #[garde(skip)]
        initial: Duration,
        /// Amount to add per failed attempt
        #[garde(custom(is_non_zero_duration))]
        increment: Duration,
    },
    // Retry at a exponentially increasing rate (5s, 10s, 20s, 40s)
    ExponentialBackoff {
        /// Initial backoff duration, must be greater than zero
        #[garde(custom(is_non_zero_duration))]
        initial: Duration,
        /// Exponent to multiply by
        #[garde(range(min = 2, max = 10))]
        exponent: u8,
    },
}

const EVENT_PLACEHOLDER: &str = "{OGUARD_EVENT}";
const EVENT_NAME_PLACEHOLDER: &str = "{OGUARD_EVENT_NAME}";
const EVENT_DESCRIPTION_PLACEHOLDER: &str = "{OGUARD_EVENT_DESCRIPTION}";

fn replace_event_placeholders(event: UPSEvent, value: &str) -> String {
    let mut value = Cow::Borrowed(value);
    let event_name = event.to_string();

    if value.contains(EVENT_PLACEHOLDER) {
        value = Cow::Owned(value.replace(EVENT_PLACEHOLDER, &event_name));
    }

    if value.contains(EVENT_NAME_PLACEHOLDER) {
        let label_key = format!("event.{}.label", event_name);
        value = Cow::Owned(value.replace(EVENT_NAME_PLACEHOLDER, &label_key));
    }

    if value.contains(EVENT_DESCRIPTION_PLACEHOLDER) {
        let description_key = format!("event.{}.description", event_name);
        value = Cow::Owned(value.replace(EVENT_DESCRIPTION_PLACEHOLDER, &description_key));
    }

    value.to_string()
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
#[cfg(target_os = "windows")]
pub async fn execute_shutdown(event: UPSEvent, config: &ShutdownAction) -> anyhow::Result<()> {
    let message = config
        .message
        .as_ref()
        .map(|value| replace_event_placeholders(event, value))
        .unwrap_or_else(|| format!("Shutdown triggered by {event} pipeline"));
    let timeout = config
        .timeout
        .map(|value| value.as_secs() as u32)
        .unwrap_or(0);
    let force_close_apps = config.force_close_apps;

    spawn_blocking(move || {
        system_shutdown::shutdown_with_message(&message, timeout, force_close_apps)
    })
    .await
    .context("failed to join shutdown task")?
    .context("failed to shutdown")?;

    Ok(())
}

/// Executes a shutdown
#[cfg(target_os = "linux")]
pub async fn execute_shutdown(_event: UPSEvent, _config: &ShutdownAction) -> anyhow::Result<()> {
    spawn_blocking(system_shutdown::shutdown)
        .await
        .context("failed to join shutdown task")?
        .context("failed to shutdown")?;

    Ok(())
}

/// Triggers the UPS to shutdown
pub async fn execute_shutdown_ups<D: Device>(
    config: &UPSShutdownAction,
    executor: &DeviceExecutorHandle<D>,
) -> anyhow::Result<()> {
    executor
        .send(ScheduleUPSShutdown {
            delay_minutes: config.delay_minutes.0,
            reboot_delay_minutes: 1,
        })
        .await
        .context("failed to schedule ups shutdown")?;

    Ok(())
}

/// Starts an executable process
pub async fn execute_executable(
    event: UPSEvent,
    executable: &ExecutableAction,
) -> anyhow::Result<()> {
    // Replace placeholder arguments
    let args: Vec<_> = executable
        .args
        .iter()
        .map(|arg| replace_event_placeholders(event, arg))
        .collect();

    let child = Command::new(&executable.exe)
        .args(&args)
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

    let mut builder = client.request(method, &request.url);

    if let Some(timeout) = request.timeout.as_ref() {
        builder = builder.timeout(*timeout);
    }

    if let Some(body) = request.body.as_ref() {
        let payload = replace_event_placeholders(event, &body.payload);
        builder = builder.body(payload);
        if let Ok(header_value) = HeaderValue::from_str(&body.content_type) {
            headers.insert(header::CONTENT_TYPE, header_value);
        }
    }

    let request = builder
        .headers(headers)
        .build()
        .context("building http request")?;

    let _response = client
        .execute(request)
        .await
        .context("error sending request")?
        .error_for_status()
        .context("response error")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{
        Action, ActionDelay, ActionPipeline, ActionType, EventPipelineModel, EventPipelineRunner,
    };
    use crate::{
        action::ExecutableAction,
        database::{connect_database, entities::events::UPSEvent},
        logging::setup_test_logging,
        services::watcher::UPSWatcherHandle,
        ups::{DeviceExecutor, HidDeviceCreator},
    };
    use chrono::Utc;
    use log::debug;
    use std::time::Duration;
    use tokio::{sync::broadcast, time::sleep};

    fn setup_tests() {
        setup_test_logging();
    }

    async fn test_pipeline(
        event: UPSEvent,
        pipeline: ActionPipeline,
        cancellable: bool,
    ) -> anyhow::Result<()> {
        let (tx, rx) = broadcast::channel(8);
        let watcher_handle = UPSWatcherHandle { rx };
        let executor = DeviceExecutor::start(HidDeviceCreator::new()?)?;

        // Use in memory database for event pipelines
        let db = connect_database("sqlite::memory:").await;

        EventPipelineModel::create(
            &db,
            "Test action".to_string(),
            event,
            pipeline,
            cancellable,
            Utc::now(),
        )
        .await?;
        debug!("spawning runner");

        tokio::spawn(EventPipelineRunner::new(db, watcher_handle, executor).run());

        debug!("sending event");

        tx.send(UPSEvent::ACFailure)?;

        // Sleep for 1 minute to allow test a chance to run
        sleep(Duration::from_secs(60)).await;

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Destructive action test"]
    async fn test_full_shutdown() {
        setup_tests();

        test_pipeline(
            UPSEvent::ACFailure,
            ActionPipeline {
                actions: vec![
                    Action {
                        ty: ActionType::Notification,
                        delay: Some(ActionDelay {
                            below_capacity: None,
                            duration: None,
                        }),
                        repeat: None,
                        retry: None,
                    },
                    // Action {
                    //     ty: ActionType::Shutdown(ShutdownAction {
                    //         message: Some("Full shutdown test".to_string()),
                    //         timeout: None,
                    //         force_close_apps: false,
                    //     }),
                    //     delay: ActionDelay {
                    //         below_capacity: None,
                    //         duration: None,
                    //     },
                    //     repeat: None,
                    //     retry: None,
                    // },
                    // Action {
                    //     ty: ActionType::USPShutdown(UPSShutdownAction { delay_minutes: 1.5 }),
                    //     delay: ActionDelay {
                    //         below_capacity: None,
                    //         duration: None,
                    //     },
                    //     repeat: None,
                    //     retry: None,
                    // },
                ],
            },
            false,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore = "Destructive action test"]
    async fn test_notification_action() {
        setup_tests();

        test_pipeline(
            UPSEvent::ACFailure,
            ActionPipeline {
                actions: vec![Action {
                    ty: ActionType::Notification,
                    delay: Some(ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    }),
                    repeat: None,
                    retry: None,
                }],
            },
            false,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore = "Destructive action test"]
    async fn test_popup_action() {
        setup_tests();

        test_pipeline(
            UPSEvent::ACFailure,
            ActionPipeline {
                actions: vec![Action {
                    ty: ActionType::Popup,
                    delay: Some(ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    }),
                    repeat: None,
                    retry: None,
                }],
            },
            false,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore = "Destructive action test"]
    async fn test_sleep_action() {
        setup_tests();

        test_pipeline(
            UPSEvent::ACFailure,
            ActionPipeline {
                actions: vec![Action {
                    ty: ActionType::Sleep,
                    delay: Some(ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    }),
                    repeat: None,
                    retry: None,
                }],
            },
            false,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore = "Destructive action test"]
    async fn test_executable_action() {
        setup_tests();

        test_pipeline(
            UPSEvent::ACFailure,
            ActionPipeline {
                actions: vec![Action {
                    ty: ActionType::Executable(ExecutableAction {
                        exe: "notepad.exe".to_string(),
                        args: vec![],
                        timeout: None,
                    }),
                    delay: Some(ActionDelay {
                        below_capacity: None,
                        duration: Some(Duration::from_secs(5)),
                    }),
                    repeat: None,
                    retry: None,
                }],
            },
            false,
        )
        .await
        .unwrap();
    }
}
