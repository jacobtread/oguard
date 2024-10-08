use axum::{
    routing::{get, post},
    Router,
};

use crate::ups::device::DefaultDevice;

mod auth;
mod history;
mod pipelines;
mod realtime;
mod server;
mod state;
mod web;

pub fn router() -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/server", get(server::server_details))
                .route("/device-state", get(state::device_state::<DefaultDevice>))
                .route(
                    "/battery-state",
                    get(state::device_battery::<DefaultDevice>),
                )
                .route("/events", get(state::events))
                .nest(
                    "/history",
                    Router::new()
                        .route("/battery-state", get(history::battery_state_history))
                        .route("/device-state", get(history::device_state_history))
                        .route("/event", get(history::event_history)),
                )
                .nest(
                    "/event-pipelines",
                    Router::new()
                        .route(
                            "/",
                            get(pipelines::get_event_pipelines)
                                .post(pipelines::create_event_pipeline),
                        )
                        .nest(
                            "/:id",
                            Router::new()
                                .route(
                                    "/",
                                    get(pipelines::get_event_pipeline)
                                        .put(pipelines::update_event_pipeline)
                                        .delete(pipelines::delete_event_pipeline),
                                )
                                .route(
                                    "/test",
                                    post(pipelines::test_event_pipeline::<DefaultDevice>),
                                ),
                        ),
                )
                .route("/toggle-buzzer", post(realtime::toggle_buzzer))
                .nest(
                    "/test-battery",
                    Router::new()
                        .route("/start", post(realtime::test_battery_start))
                        .route("/cancel", post(realtime::test_battery_cancel)),
                )
                .route("/login", post(auth::login))
                .route("/login-state", get(auth::login_state))
                .route("/logout", post(auth::logout)),
        )
        // Public content fallback
        .fallback_service(web::PublicContent)
}
