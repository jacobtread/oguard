use garde::Validate;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

use crate::{
    action::ActionPipeline, database::entities::events::UPSEvent, utils::validate::valid_range,
};

#[derive(Debug, Validate, Deserialize)]
pub struct RangeQuery {
    #[garde(skip)]
    pub start: DateTimeUtc,
    #[garde(custom(valid_range(&self.start)))]
    pub end: DateTimeUtc,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateEventPipeline {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub event: UPSEvent,
    #[garde(dive)]
    pub pipeline: ActionPipeline,
    #[garde(skip)]
    pub cancellable: bool,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateEventPipeline {
    #[garde(inner(length(min = 1)))]
    pub name: Option<String>,
    #[garde(skip)]
    pub event: Option<UPSEvent>,
    #[garde(dive)]
    pub pipeline: Option<ActionPipeline>,
    #[garde(skip)]
    pub cancellable: Option<bool>,
    #[garde(skip)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[garde(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginStateResponse {
    pub logged_in: bool,
}
