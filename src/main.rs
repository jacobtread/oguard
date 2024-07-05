//! Name reserved for project    

use std::time::Duration;

use log::debug;
use notify_rust::Notification;
use tokio::time::sleep;
use ups::{QueryDeviceBattery, UPSExecutor};
use watcher::UPSWatcher;

pub mod config;
pub mod ups;
pub mod watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    // Start the executor
    let executor = UPSExecutor::start()?;

    // Start a watcher
    let mut watcher_handle = UPSWatcher::start(executor.clone());

    tokio::spawn(async move {
        while let Some(value) = watcher_handle.next().await {
            debug!("UPS EVENT {:#?}", value);

            let watcher::UPSWatcherMessage::Event(event) = value;
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

    loop {
        let battery = executor.request(QueryDeviceBattery).await?;
        debug!("Obtained device battery: {:#?}", battery);
        sleep(Duration::from_secs(5)).await;
    }
}
