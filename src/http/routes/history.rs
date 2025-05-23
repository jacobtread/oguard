use crate::{
    database::entities::{
        battery_history::BatteryHistoryModel, events::EventModel, state_history::StateHistoryModel,
    },
    http::{error::HttpResult, models::RangeQuery},
};
use anyhow::Context;
use axum::extract::Query;
use axum::{Extension, Json};
use axum_valid::Garde;
use sea_orm::DatabaseConnection;

/// GET /api/history/battery-state
///
/// Get the battery state history for the provided date range
pub async fn battery_state_history(
    Extension(db): Extension<DatabaseConnection>,
    Garde(Query(RangeQuery { start, end })): Garde<Query<RangeQuery>>,
) -> HttpResult<Vec<BatteryHistoryModel>> {
    let history = BatteryHistoryModel::get_range(&db, start, end)
        .await
        .context("Failed to query battery history")?;

    Ok(Json(history))
}

/// GET /api/history/device-state
///
/// Get the device state history for the provided date range
pub async fn device_state_history(
    Extension(db): Extension<DatabaseConnection>,
    Garde(Query(RangeQuery { start, end })): Garde<Query<RangeQuery>>,
) -> HttpResult<Vec<StateHistoryModel>> {
    let history = StateHistoryModel::get_range(&db, start, end)
        .await
        .context("Failed to query state history")?;

    Ok(Json(history))
}

/// GET /api/history/event
///
/// Get the device state history for the provided date range
pub async fn event_history(
    Extension(db): Extension<DatabaseConnection>,
    Garde(Query(RangeQuery { start, end })): Garde<Query<RangeQuery>>,
) -> HttpResult<Vec<EventModel>> {
    let history = EventModel::get_range(&db, start, end)
        .await
        .context("Failed to query event history")?;

    Ok(Json(history))
}
