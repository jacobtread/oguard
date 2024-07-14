//! Models used for parsing command responses

use ordered_float::OrderedFloat;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

/// Current device state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct DeviceState {
    /// Voltage going into the UPS (Power coming from wall)
    pub input_voltage: OrderedFloat<f64>,
    /// Voltage coming out of the UPS (Power coming from UPS)
    pub output_voltage: OrderedFloat<f64>,
    /// Percentage load/usage of the UPS
    pub output_load_percent: u8,
    /// Output frequency from the UPS
    pub output_frequency: OrderedFloat<f64>,
    /// Voltage of the battery
    pub battery_voltage: OrderedFloat<f64>,
    /// Current source of power
    pub device_power_state: DevicePowerState,
    /// Low battery state
    pub battery_low: bool,
    /// Fault state
    pub fault_mode: bool,
    /// Device line type
    pub device_line_type: DeviceLineType,
    /// Device self test state
    pub battery_self_test: bool,
    /// Buzzer controller state
    pub buzzer_control: bool,
}

impl DeviceState {
    /// Get the "Work Mode" aka current state of the device
    pub fn get_work_mode(&self) -> WorkMode {
        if self.fault_mode {
            return WorkMode::Fault;
        }

        if self.output_voltage < OrderedFloat(20.0) {
            return WorkMode::Standby;
        }

        match (self.device_power_state, self.battery_self_test) {
            // Running from utility power and in testing state
            (DevicePowerState::Utility, true) => WorkMode::BatteryTest,
            // Running from utility power
            (DevicePowerState::Utility, false) => WorkMode::Line,
            // Running from battery
            (DevicePowerState::Battery, _) => WorkMode::Battery,
        }
    }
}

/// "Work mode" - Possible current states the UPS is in
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum WorkMode {
    /// UPS is in standby mode, running with less than
    /// 20v on input
    Standby,
    /// UPS is running from utility line power
    Line,
    /// UPS is in battery testing mode
    BatteryTest,
    /// UPS is running from battery
    Battery,
    /// UPS has encountered a fault
    Fault,
}

impl WorkMode {
    pub fn is_battery(&self) -> bool {
        matches!(self, Self::Battery)
    }
}

/// UPS device line type
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceLineType {
    /// Device is line interactive
    LineInteractive,
    /// Device is on-line
    OnLine,
}

/// Current source of power for the UPS
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum DevicePowerState {
    /// Device is being powered from a socket, battery is not used
    Utility,
    /// No power coming from socket, battery is in use
    Battery,
}

/// Response from a device battery query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult)]
pub struct DeviceBattery {
    /// Capacity of the battery as a percentage 0-100
    pub capacity: u8,
    /// Remaining time of the battery charge in seconds
    pub remaining_time: u32,
}
