use axum::{routing::get, Router};

pub mod error;
mod routes;

pub fn router() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/device-state", get(routes::device_state))
            .route("/battery-state", get(routes::device_battery))
            .route("/events", get(routes::events)),
    )
}
