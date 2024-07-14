use crate::ups::{BatteryTest, CancelBatteryTest, DeviceExecutorHandle, ToggleBuzzer};
use anyhow::Context;
use axum::Extension;
use reqwest::StatusCode;

use crate::http::error::HttpStatusResult;
use crate::http::middleware::auth_gate::AuthGate;

/// POST /api/toggle-buzzer
///
/// Toggle the UPS buzzer state
pub async fn toggle_buzzer(
    _: AuthGate,
    Extension(executor): Extension<DeviceExecutorHandle>,
) -> HttpStatusResult {
    executor
        .send(ToggleBuzzer)
        .await
        .context("toggle buzzer request")?;

    Ok(StatusCode::OK)
}

/// POST /api/test-battery/start
///
/// Starts a 10s battery test
pub async fn test_battery_start(
    _: AuthGate,
    Extension(executor): Extension<DeviceExecutorHandle>,
) -> HttpStatusResult {
    executor
        .send(BatteryTest)
        .await
        .context("battery test request")?;

    Ok(StatusCode::OK)
}

/// POST /api/test-battery/cancel
///
/// Cancels a 10s battery test
pub async fn test_battery_cancel(
    _: AuthGate,
    Extension(executor): Extension<DeviceExecutorHandle>,
) -> HttpStatusResult {
    executor
        .send(CancelBatteryTest)
        .await
        .context("cancel battery test request")?;

    Ok(StatusCode::OK)
}
