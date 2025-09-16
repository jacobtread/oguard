use axum::Json;

use crate::{
    VERSION,
    http::{error::HttpResult, models::ServerDetails},
};

/// GET /api/server
///
/// Responds with details about the oguard server
pub async fn server_details() -> HttpResult<ServerDetails> {
    Ok(Json(ServerDetails { version: VERSION }))
}
