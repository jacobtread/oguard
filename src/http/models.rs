use sea_orm::prelude::DateTimeUtc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RangeQuery {
    pub start: DateTimeUtc,
    pub end: DateTimeUtc,
}
