//! Name reserved for project    

use std::time::Duration;

use log::debug;
use tokio::time::sleep;
use ups::UPSExecutor;
use watcher::UPSWatcher;

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
        }
    });

    loop {
        let battery = executor.device_battery().await?;
        debug!("Obtained device battery: {:#?}", battery);
        sleep(Duration::from_secs(5)).await;
    }
}
