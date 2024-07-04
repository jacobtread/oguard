//! Name reserved for project    

use std::time::Duration;

use tokio::time::sleep;
use ups::UPSDevice;

pub mod ups;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = UPSDevice::start()?;

    let mut listener_handle = handle.duplicate();

    tokio::spawn(async move {
        while let Ok(value) = listener_handle.rx.recv().await {
            println!("UPS EVENT {:#?}", value);
        }
    });

    loop {
        let battery = handle.query_device_battery().await?;
        println!("Obtained device battery: {:#?}", battery);
        sleep(Duration::from_secs(5)).await;
    }
}
