use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::{action::ActionPipeline, database::entities::events::UPSEvent};

#[derive(Debug, Deserialize)]
pub struct RangeQuery {
    pub start: DateTimeUtc,
    pub end: DateTimeUtc,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventPipeline {
    pub name: String,
    pub event: UPSEvent,
    pub pipeline: ActionPipeline,
    pub cancellable: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventPipeline {
    pub name: Option<String>,
    pub pipeline: Option<ActionPipeline>,
    pub cancellable: Option<bool>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginStateResponse {
    pub logged_in: bool,
}
