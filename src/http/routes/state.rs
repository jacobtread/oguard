use crate::{
    http::error::HttpResult,
    services::watcher::UPSWatcherHandle,
    ups::{
        device::Device, DeviceBattery, DeviceExecutorHandle, DeviceState, QueryDeviceBattery,
        QueryDeviceState,
    },
};
use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    Extension, Json,
};
use futures::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt;

/// GET /api/device-state
///
/// Requests the current state of the device
pub async fn device_state<D: Device>(
    Extension(executor): Extension<DeviceExecutorHandle<D>>,
) -> HttpResult<DeviceState> {
    let device_state = executor.send(QueryDeviceState).await?;

    Ok(Json(device_state))
}

/// GET /api/battery-state
///
/// Requests the current battery capacity and remaining duration
pub async fn device_battery<D: Device>(
    Extension(executor): Extension<DeviceExecutorHandle<D>>,
) -> HttpResult<DeviceBattery> {
    let battery = executor.send(QueryDeviceBattery).await?;

    Ok(Json(battery))
}

/// GET /api/events
///
/// SSE events endpoint
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

#[cfg(test)]
mod test {
    use axum::Extension;

    use super::{device_battery, device_state};
    use crate::ups::{DeviceExecutor, MockDevice, MockDeviceCreator};

    /// Tests that the device_state endpoint executes the correct command
    /// and provides a success response for a valid device response
    #[tokio::test]
    async fn test_device_state() {
        let (device_creator, mut mock_handle) = MockDeviceCreator::new();
        let executor = DeviceExecutor::<MockDevice>::start(device_creator).unwrap();

        // Set the battery response
        mock_handle.next_response("(237.1 237.1 237.1 008 50.1 27.1 --.- 00001001".into());

        _ = device_state(Extension(executor))
            .await
            .expect("failed to get device state");

        // Get the command that was executed
        let command = mock_handle.next_command().await;

        // Ensure expected command was executed
        assert_eq!(command, Some("QS".into()));
    }

    /// Tests that the device_battery endpoint executes the correct command
    /// and provides a success response for a valid device response
    #[tokio::test]
    async fn test_device_battery() {
        let (device_creator, mut mock_handle) = MockDeviceCreator::new();
        let executor = DeviceExecutor::<MockDevice>::start(device_creator).unwrap();

        // Set the battery response
        mock_handle.next_response("(100 02832 50.0 000.5 175 290 0 0000020000112000".into());

        _ = device_battery(Extension(executor))
            .await
            .expect("failed to get device state");

        // Get the command that was executed
        let command = mock_handle.next_command().await;

        // Ensure expected command was executed
        assert_eq!(command, Some("QI".into()));
    }
}
