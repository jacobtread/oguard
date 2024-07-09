use sea_orm::prelude::DateTimeUtc;
use serde::Deserialize;

use crate::{action::ActionPipeline, database::entities::events::UPSEvent};

#[derive(Debug, Deserialize)]
pub struct RangeQuery {
    pub start: DateTimeUtc,
    pub end: DateTimeUtc,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventPipeline {
    pub event: UPSEvent,
    pub pipelines: Vec<ActionPipeline>,
    pub cancellable: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventPipeline {
    pub pipelines: Vec<ActionPipeline>,
    pub cancellable: bool,
}
