use axum::Extension;
use http::router;
use log::debug;
use notify_rust::Notification;
use ups::UPSExecutor;
use watcher::{UPSWatcher, UPSWatcherHandle};

pub mod config;
pub mod database;
pub mod http;
pub mod ups;
pub mod watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    // Start the executor
    let executor = UPSExecutor::start()?;

    // Start a watcher
    let watcher_handle = UPSWatcher::start(executor.clone());

    spawn_tray_listener(watcher_handle.clone());

    // build our application with a single route
    let app = router()
        .layer(Extension(executor))
        .layer(Extension(watcher_handle));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

/// Spawns an event listener for publishing notifications when the
/// watcher detects a change in state
fn spawn_tray_listener(mut watcher_handle: UPSWatcherHandle) {
    tokio::spawn(async move {
        while let Some(event) = watcher_handle.next().await {
            debug!("UPS EVENT {:#?}", event);

            match event {
                watcher::UPSEvent::ACFailure => {
                    _ = Notification::new()
                        .summary("AC Power Failure")
                        .body("No longer receiving AC power, running on battery")
                        .icon("dialog-negative")
                        .show();
                }
                watcher::UPSEvent::ACRecovery => {
                    _ = Notification::new()
                        .summary("AC Power Recovered")
                        .body("Receiving AC power, no longer running on battery")
                        .icon("dialog-positive")
                        .show();
                }
                watcher::UPSEvent::UPSFault => {
                    _ = Notification::new()
                        .summary("Fault encountered")
                        .body("UPS Encountered a fault")
                        .icon("dialog-negative")
                        .show();
                }
                watcher::UPSEvent::LowBatteryModeStart => {
                    _ = Notification::new()
                        .summary("UPS Low Battery")
                        .body("UPS is running low on battery")
                        .icon("dialog-negative")
                        .show();
                }
                watcher::UPSEvent::LowBatteryModeEnd => {
                    _ = Notification::new()
                        .summary("UPS Sufficient Battery")
                        .body("UPS is no longer running low on battery")
                        .icon("dialog-positive")
                        .show();
                }
                watcher::UPSEvent::BatteryTestStart => {
                    _ = Notification::new()
                        .summary("UPS Battery Test Started")
                        .body("UPS is testing the battery")
                        .icon("dialog-positive")
                        .show();
                }
                watcher::UPSEvent::BatteryTestEnd => {
                    _ = Notification::new()
                        .summary("UPS Battery Test Finished")
                        .body("UPS has finished testing the battery")
                        .icon("dialog-positive")
                        .show();
                }
            }
        }
    });
}
