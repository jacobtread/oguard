use axum::{routing::get, Router};

pub mod error;
pub mod models;
mod routes;

pub fn router() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/device-state", get(routes::device_state))
            .route("/battery-state", get(routes::device_battery))
            .route("/events", get(routes::events))
            .nest(
                "/history",
                Router::new()
                    .route("/battery-state", get(routes::battery_state_history))
                    .route("/device-state", get(routes::device_state_history))
                    .route("/event", get(routes::event_history)),
            )
            .nest(
                "/event-pipelines",
                Router::new()
                    .route(
                        "/",
                        get(routes::get_event_pipelines).post(routes::create_event_pipeline),
                    )
                    .route(
                        "/:id",
                        get(routes::get_event_pipeline).put(routes::update_event_pipeline),
                    ),
            ),
    )
}
