use crate::config::Config;
use crate::database;
use crate::http::router;
use crate::services::event_tracker::UPSEventTracker;
use crate::services::history_tracker::UPSHistoryTracker;
use crate::ups::UPSExecutor;
use crate::watcher::{UPSWatcher, UPSWatcherHandle};
use crate::{action::EventPipelineRunner, ups::UPSExecutorHandle};
use axum::{http::HeaderValue, Extension};
use axum_session::{Key, SessionConfig, SessionLayer, SessionMode, SessionNullPool, SessionStore};
use log::debug;
use reqwest::{header, Method};
use rust_i18n::t;
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

/// Starts and runs the app server until the `shutdown_rx` receives a message
pub async fn run_server(config: Config, shutdown_rx: mpsc::Receiver<()>) -> anyhow::Result<()> {
    let config = Arc::new(config);

    // Set current locale
    rust_i18n::set_locale(&config.locale);

    // Connect to the database
    let database = database::init().await;

    // Start the executor
    let executor = UPSExecutor::start()?;

    // Start an event watcher
    let watcher_handle = UPSWatcher::start(executor.clone());

    // Start background services
    start_services(&database, &executor, &watcher_handle);

    // Create in memory session store
    let session_store = SessionStore::<SessionNullPool>::new(
        None,
        SessionConfig::default()
            .with_key(Key::generate())
            .with_mode(SessionMode::OptIn),
    )
    .await?;

    // Create the address to bind the server on
    let address = SocketAddr::new(config.http.host, config.http.port);

    // build our application with a single route
    let mut app = router()
        .layer(SessionLayer::new(session_store))
        .layer(Extension(database))
        .layer(Extension(executor))
        .layer(Extension(watcher_handle))
        .layer(Extension(config));

    // CORS layer required for development access
    #[cfg(debug_assertions)]
    {
        app = app.layer(debug_cors_layer());
    }

    // Bind the TCP listener for the HTTP server
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // Log the startup message
    debug!(
        "{}",
        t!("server.started", host = format!("http://{}", address))
    );

    // Serve the app
    axum::serve(listener, app)
        // Attach graceful shutdown to the shutdown receiver
        .with_graceful_shutdown(async move {
            let mut shutdown_rx = shutdown_rx;
            _ = shutdown_rx.recv().await;
        })
        .await?;

    Ok(())
}

/// Starts background services that depend on the app resources
fn start_services(
    database: &DatabaseConnection,
    executor: &UPSExecutorHandle,
    watcher_handle: &UPSWatcherHandle,
) {
    // Start long term watcher that logs state to database
    UPSHistoryTracker::start(database.clone(), executor.clone());

    // Start the event tracker
    UPSEventTracker::start(database.clone(), watcher_handle.clone());

    // Start the event pipeline runner
    EventPipelineRunner::start(database.clone(), watcher_handle.clone(), executor.clone());
}

/// CORS Layer required in development mode where the web server is
/// served through a separate dev server
#[cfg(debug_assertions)]
fn debug_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::ACCEPT, header::CONTENT_TYPE])
        .allow_origin(
            "http://localhost:5173"
                .parse::<HeaderValue>()
                .expect("origin was not valid"),
        )
        .allow_credentials(true)
}
