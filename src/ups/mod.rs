//! # UPS
//!
//! Modules related to interacting with the UPS itself, commands executors and device
//! handle management.

pub mod command;
pub mod commands;
pub mod device;
pub mod executor;
pub mod models;

pub use commands::*;
pub use device::HidDeviceCreator;
pub use executor::{DeviceExecutor, DeviceExecutorHandle};
pub use models::*;

#[cfg(test)]
pub use device::test::*;
