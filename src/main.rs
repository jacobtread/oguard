use action::EventPipelineRunner;
use axum::Extension;
use axum_session::{Key, SessionConfig, SessionLayer, SessionMode, SessionNullPool, SessionStore};
use chrono::Utc;
use database::entities::events::EventModel;
use http::router;
use log::{debug, error};
use persistent_watcher::UPSPersistentWatcher;
use rust_i18n::{i18n, t};
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use ups::UPSExecutor;
use watcher::{UPSWatcher, UPSWatcherHandle};

pub mod action;
pub mod config;
pub mod database;
pub mod http;
pub mod persistent_watcher;
pub mod ups;
pub mod watcher;

i18n!("locales", fallback = "en");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();
    log_panics::init();

    // Load the configuration
    let config = config::load_default().await;
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
    let app = router()
        .layer(CorsLayer::permissive())
        .layer(SessionLayer::new(session_store))
        .layer(Extension(database))
        .layer(Extension(executor))
        .layer(Extension(watcher_handle))
        .layer(Extension(config.clone()));

    // Create the address to bind the server on
    let address = SocketAddr::new(config.http.host, config.http.port);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    debug!(
        "{}",
        t!("server.started", host = format!("http://{}", address))
    );

    axum::serve(listener, app).await?;

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
