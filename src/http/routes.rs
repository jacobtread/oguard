use std::convert::Infallible;
use std::time::Duration;

use crate::database::entities::battery_history::BatteryHistoryModel;
use crate::database::entities::events::EventModel;
use crate::database::entities::state_history::StateHistoryModel;
use crate::http::error::HttpResult;
use crate::ups::{
    DeviceBattery, DeviceState, QueryDeviceBattery, QueryDeviceState, UPSExecutorHandle,
};
use crate::watcher::UPSWatcherHandle;
use anyhow::Context;
use axum::extract::Query;
use axum::response::sse::{Event, KeepAlive};
use axum::response::Sse;
use axum::{Extension, Json};
use futures::Stream;
use sea_orm::DatabaseConnection;
use tokio_stream::StreamExt;

use super::models::RangeQuery;

/// GET /api/device-state
///
/// Requests the current state of the device
pub async fn device_state(
    Extension(executor): Extension<UPSExecutorHandle>,
) -> HttpResult<DeviceState> {
    let battery = executor.request(QueryDeviceState).await?;

    Ok(Json(battery))
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
