use crate::action::EventPipelineRunner;
use crate::config::Config;
use crate::database;
use crate::http::router;
use crate::persistent_watcher::UPSPersistentWatcher;
use crate::ups::UPSExecutor;
use crate::watcher::{UPSWatcher, UPSWatcherHandle};
use axum::{http::HeaderValue, Extension};
use axum_session::{Key, SessionConfig, SessionLayer, SessionMode, SessionNullPool, SessionStore};
use chrono::Utc;
use database::entities::events::EventModel;
use log::{debug, error};
use reqwest::{header, Method};
use rust_i18n::t;
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

pub async fn run_server(config: Config, shutdown_rx: mpsc::Receiver<()>) -> anyhow::Result<()> {
    let config = Arc::new(config);

    // Set current locale
    rust_i18n::set_locale(&config.locale);

    // Connect to the database
    let database = database::init().await;

    // Start the executor
    let executor = UPSExecutor::start()?;

    // Start long term watcher that logs state to database
    UPSPersistentWatcher::start(executor.clone(), database.clone());

    // Start a watcher
    let watcher_handle = UPSWatcher::start(executor.clone());

    // Spawn event watch listeners
    spawn_persist_listener(watcher_handle.clone(), database.clone());

    // Start the event pipeline runner
    let event_pipeline_runner =
        EventPipelineRunner::new(executor.clone(), database.clone(), watcher_handle.clone());
    tokio::spawn(event_pipeline_runner.run());

    let session_config = SessionConfig::default()
        .with_key(Key::generate())
        .with_mode(SessionMode::OptIn);

    let session_store = SessionStore::<SessionNullPool>::new(None, session_config).await?;

    // build our application with a single route
    let mut app = router()
        .layer(SessionLayer::new(session_store))
        .layer(Extension(database))
        .layer(Extension(executor))
        .layer(Extension(watcher_handle))
        .layer(Extension(config.clone()));

    // CORS layer required for development access
    #[cfg(debug_assertions)]
    {
        app = app.layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers([header::ACCEPT, header::CONTENT_TYPE])
                .allow_origin("http://localhost:5173".parse::<HeaderValue>()?)
                .allow_credentials(true),
        )
    }

    // Create the address to bind the server on
    let address = SocketAddr::new(config.http.host, config.http.port);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    debug!(
        "{}",
        t!("server.started", host = format!("http://{}", address))
    );

    axum::serve(listener, app)
        // Attach graceful shutdown to the shutdown receiver
        .with_graceful_shutdown(async move {
            let mut shutdown_rx = shutdown_rx;
            _ = shutdown_rx.recv().await;
        })
        .await?;

    Ok(())
}

/// Spawns an event listener that persists events to the database
fn spawn_persist_listener(mut watcher_handle: UPSWatcherHandle, db: DatabaseConnection) {
    tokio::spawn(async move {
        while let Some(event) = watcher_handle.next().await {
            let current_time = Utc::now();
            if let Err(err) = EventModel::create(&db, event, current_time).await {
                error!("failed to save event to database: {err}");
            }
        }
    });
}
