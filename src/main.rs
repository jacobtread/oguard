use std::net::{Ipv4Addr, SocketAddrV4};

use axum::Extension;
use chrono::Utc;
use database::entities::events::{EventModel, EventType};
use http::router;
use log::{debug, error};
use notify_rust::Notification;
use persistent_watcher::UPSPersistentWatcher;
use rust_i18n::{i18n, t};
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;
use ups::UPSExecutor;
use watcher::{UPSEvent, UPSWatcher, UPSWatcherHandle};

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

    // Set current locale
    rust_i18n::set_locale("en");

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
    spawn_tray_listener(watcher_handle.clone());

    // build our application with a single route
    let app = router()
        .layer(CorsLayer::permissive())
        .layer(Extension(database))
        .layer(Extension(executor))
        .layer(Extension(watcher_handle));

    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3000);

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
            if let Err(err) = EventModel::create(&db, EventType::from(event), current_time).await {
                error!("failed to save event to database: {err}");
            }
        }
    });
}

/// Spawns an event listener for publishing notifications when the
/// watcher detects a change in state
fn spawn_tray_listener(mut watcher_handle: UPSWatcherHandle) {
    tokio::spawn(async move {
        while let Some(event) = watcher_handle.next().await {
            debug!("UPS EVENT {:#?}", event);
            send_event_notification(event);
        }
    });
}

/// Sends a notification for the provided event
fn send_event_notification(event: UPSEvent) {
    let event_name = event.to_string();

    let label_key = format!("event.{}.label", event_name);
    let description_key = format!("event.{}.description", event_name);

    let label = t!(&label_key);
    let description = t!(&description_key);

    let icon = match event {
        watcher::UPSEvent::ACFailure => "dialog-negative",
        watcher::UPSEvent::ACRecovery => "dialog-positive",
        watcher::UPSEvent::UPSFault => "dialog-negative",
        _ => "dialog-positive",
    };

    _ = Notification::new()
        .summary(&label)
        .body(&description)
        .icon(icon)
        .show();
}
