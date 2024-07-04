//! Name reserved for project    

use std::error::Error;

use anyhow::Context;
use hidapi::{HidApi, HidDevice};

fn main() -> Result<(), Box<dyn Error>> {
    const VENDOR_ID: u16 = 0x0665;
    const PRODUCT_ID: u16 = 0x5161;

    let api = HidApi::new().unwrap();

    let mut device = api
        .open(VENDOR_ID, PRODUCT_ID)
        .expect("Failed to open device");

    let battery = query_device_battery(&mut device)?;

    println!("Response: {battery:?}");

    Ok(())
}

/// Response from a device battery query
#[derive(Debug)]
pub struct DeviceBattery {
    /// Capacity of the battery as a percentage 0-100
    pub capacity: u8,
    /// Remaining time of the battery charge in seconds
    pub remaining_time: u32,
}

fn query_device_battery(device: &mut HidDevice) -> anyhow::Result<DeviceBattery> {
    // 100 02832 50.0 000.5 175 290 0 0000020000112000

    execute_command(device, "QI")?;
    let response = read_response(device)?;

    let response = response
        .strip_prefix('(')
        .context("Missing device battery response prefix")?;

    let mut parts = response.split(' ');

    let capacity_str = parts.next().context("Missing capacity value")?;
    let remaining_str = parts.next().context("Missing battery remaining value")?;

    let capacity = capacity_str.parse().context("Invalid capacity value")?;
    let remaining_time = remaining_str
        .parse()
        .context("Invalid battery remaining value")?;

    Ok(DeviceBattery {
        capacity,
        remaining_time,
    })
}

fn execute_command(device: &mut HidDevice, cmd: &str) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    buffer.push(0); // Report ID
    buffer.extend_from_slice(cmd.as_bytes());
    buffer.push(b'\r');
    device.write(&buffer)?;
    Ok(())
}

fn read_response(device: &mut HidDevice) -> anyhow::Result<String> {
    let mut out = String::new();

    let mut buffer = [0u8; 128];

    // TODO: Read timeout of 3000ms
    loop {
        let count = device
            .read(&mut buffer)
            .context("Failed to read response")?;

        if count == 0 {
            return Ok(out);
        }

        let chars = buffer
            .iter()
            // Take only available length
            .take(count)
            // Bytes are characters
            .map(|value| *value as char);

        for char in chars {
            // Break response at carriage return
            if char == '\r' {
                return Ok(out);
            }

            out.push(char);
        }
    }
}
