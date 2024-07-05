use std::convert::Infallible;
use std::time::Duration;

use crate::http::error::HttpResult;
use crate::ups::{
    DeviceBattery, DeviceState, QueryDeviceBattery, QueryDeviceState, UPSExecutorHandle,
};
use crate::watcher::UPSWatcherHandle;
use axum::response::sse::{Event, KeepAlive};
use axum::response::Sse;
use axum::{Extension, Json};
use futures::Stream;
use tokio_stream::StreamExt;

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
