use std::convert::Infallible;
use std::time::Duration;

use crate::database::entities::battery_history::BatteryHistoryModel;
use crate::database::entities::event_pipeline::{
    EventPipelineId, EventPipelineModel, ListEventPipeline,
};
use crate::database::entities::events::EventModel;
use crate::database::entities::state_history::StateHistoryModel;
use crate::http::error::HttpResult;
use crate::ups::{
    DeviceBattery, DeviceState, QueryDeviceBattery, QueryDeviceState, ToggleBuzzer,
    UPSExecutorHandle,
};
use crate::watcher::UPSWatcherHandle;
use anyhow::{anyhow, Context};
use axum::extract::{Path, Query};
use axum::response::sse::{Event, KeepAlive};
use axum::response::Sse;
use axum::{Extension, Json};
use chrono::Utc;
use futures::Stream;
use log::debug;
use reqwest::StatusCode;
use sea_orm::DatabaseConnection;
use tokio_stream::StreamExt;

use super::error::HttpStatusResult;
use super::models::{CreateEventPipeline, RangeQuery, UpdateEventPipeline};

/// GET /api/device-state
///
/// Requests the current state of the device
pub async fn device_state(
    Extension(executor): Extension<UPSExecutorHandle>,
) -> HttpResult<DeviceState> {
    let device_state = executor.request(QueryDeviceState).await?;

    Ok(Json(device_state))
}

/// GET /api/battery-state
///
/// Requests the current battery capacity and remaining duration
pub async fn device_battery(
    Extension(executor): Extension<UPSExecutorHandle>,
) -> HttpResult<DeviceBattery> {
    let battery = executor.request(QueryDeviceBattery).await?;

    Ok(Json(battery))
}

/// GET /api/history/battery-state
///
/// Requests the battery state history for the provided range
pub async fn battery_state_history(
    Extension(db): Extension<DatabaseConnection>,
    Query(RangeQuery { start, end }): Query<RangeQuery>,
) -> HttpResult<Vec<BatteryHistoryModel>> {
    let history = BatteryHistoryModel::get_range(&db, start, end)
        .await
        .context("Failed to query battery history")?;

    Ok(Json(history))
}

/// GET /api/history/device-state
///
/// Requests the device state history for the provided range
pub async fn device_state_history(
    Extension(db): Extension<DatabaseConnection>,
    Query(RangeQuery { start, end }): Query<RangeQuery>,
) -> HttpResult<Vec<StateHistoryModel>> {
    let history = StateHistoryModel::get_range(&db, start, end)
        .await
        .context("Failed to query state history")?;

    Ok(Json(history))
}

/// GET /api/history/event
///
/// Requests the device state history for the provided range
pub async fn event_history(
    Extension(db): Extension<DatabaseConnection>,
    Query(RangeQuery { start, end }): Query<RangeQuery>,
) -> HttpResult<Vec<EventModel>> {
    let history = EventModel::get_range(&db, start, end)
        .await
        .context("Failed to query event history")?;

    Ok(Json(history))
}

/// GET /api/event-pipelines
///
/// Requests all the event pipelines
pub async fn get_event_pipelines(
    Extension(db): Extension<DatabaseConnection>,
) -> HttpResult<Vec<ListEventPipeline>> {
    let event_pipelines = EventPipelineModel::all(&db)
        .await
        .context("failed to query event pipelines")?;

    Ok(Json(event_pipelines))
}

/// GET /api/event-pipelines/:id
///
/// Requests a specific event pipeline
pub async fn get_event_pipeline(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<EventPipelineId>,
) -> HttpResult<EventPipelineModel> {
    let event_pipeline = EventPipelineModel::find_by_id(&db, id)
        .await
        .context("failed to find event pipeline")?
        .ok_or(anyhow!("unknown event pipeline"))?;

    Ok(Json(event_pipeline))
}

/// PUT /api/event-pipelines/:id
///
/// Updates a event pipeline
pub async fn update_event_pipeline(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<EventPipelineId>,
    Json(request): Json<UpdateEventPipeline>,
) -> HttpResult<EventPipelineModel> {
    let event_pipeline = EventPipelineModel::find_by_id(&db, id)
        .await
        .context("failed to find event pipeline")?
        .ok_or(anyhow!("unknown event pipeline"))?;

    let event_pipeline = event_pipeline
        .update(
            &db,
            request.name,
            request.pipeline,
            request.cancellable,
            request.enabled,
        )
        .await
        .context("failed to update pipeline")?;

    Ok(Json(event_pipeline))
}

/// POST /api/event-pipelines
///
/// Creates a new event pipeline
pub async fn create_event_pipeline(
    Extension(db): Extension<DatabaseConnection>,
    Json(request): Json<CreateEventPipeline>,
) -> HttpResult<EventPipelineModel> {
    let current_time = Utc::now();
    let event_pipeline = EventPipelineModel::create(
        &db,
        request.name,
        request.event,
        request.pipeline,
        request.cancellable,
        current_time,
    )
    .await
    .context("failed to find event pipeline")?;

    Ok(Json(event_pipeline))
}

/// GET /api/events
///
/// Websocket hook for receiving events
pub async fn events(
    Extension(watcher_handle): Extension<UPSWatcherHandle>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = watcher_handle
        .into_stream()
        .map(|result| {
            let event = result?;
            Ok::<Event, anyhow::Error>(Event::default().json_data(event)?)
        })
        // Filter out actual failures
        .filter_map(|result| result.ok())
        // Include responses
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// POST /api/toggle-buzzer
///
/// Toggle the UPS buzzer state
pub async fn toggle_buzzer(Extension(executor): Extension<UPSExecutorHandle>) -> HttpStatusResult {
    executor
        .request(ToggleBuzzer)
        .await
        .context("toggle buzzer request")?;

    Ok(StatusCode::OK)
}
