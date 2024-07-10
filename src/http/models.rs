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
    pub name: String,
    pub event: UPSEvent,
    pub pipelines: Vec<ActionPipeline>,
    pub cancellable: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventPipeline {
    pub name: String,
    pub pipelines: Vec<ActionPipeline>,
    pub cancellable: bool,
}
