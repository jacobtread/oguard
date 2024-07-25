use axum::Json;

use crate::{
    http::{error::HttpResult, models::ServerDetails},
    VERSION,
};

/// GET /api/server
///
/// Responds with details about the oguard server
pub async fn server_details() -> HttpResult<ServerDetails> {
    Ok(Json(ServerDetails { version: VERSION }))
}
