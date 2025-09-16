use anyhow::Context;
use compact_str::{CompactString, format_compact};
use ordered_float::OrderedFloat;

use super::{
    command::{FromDeviceResponse, IntoDeviceCommand, ResponseCache},
    models::{DeviceBattery, DeviceLineType, DevicePowerState, DeviceState},
};

/// Query command to load the device battery
pub struct QueryDeviceBattery;

impl IntoDeviceCommand for QueryDeviceBattery {
    type Response = DeviceBattery;

    fn get_command(&self) -> CompactString {
        "QI".into()
    }

    fn cache_key(&self) -> Option<u64> {
        Some(0)
    }
}

impl FromDeviceResponse for DeviceBattery {
    fn from_device_response(msg: CompactString) -> anyhow::Result<Self> {
        // 100 02832 50.0 000.5 175 290 0 0000020000112000
        let msg: &str = msg
            .strip_prefix('(')
            .context("Missing device battery response prefix")?;

        let mut parts = msg.split(' ');

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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExecuteResponse {
    Success,
    Failure,
}

impl FromDeviceResponse for ExecuteResponse {
    fn from_device_response(msg: CompactString) -> anyhow::Result<Self> {
        let msg: &str = msg
            .strip_prefix('(')
            .context("Missing execute response prefix")?;

        if msg == "ACK" {
            return Ok(ExecuteResponse::Success);
        }

        Ok(ExecuteResponse::Failure)
    }
}

/// Query command to load the device state
pub struct QueryDeviceState;

impl IntoDeviceCommand for QueryDeviceState {
    type Response = DeviceState;

    fn get_command(&self) -> CompactString {
        "QS".into()
    }

    fn cache_key(&self) -> Option<u64> {
        Some(1)
    }
}

impl FromDeviceResponse for DeviceState {
    fn from_device_response(msg: CompactString) -> anyhow::Result<Self> {
        let msg = msg
            .strip_prefix('(')
            .context("Missing device battery response prefix")?;

        let mut parts = msg.split(' ');

        let input_voltage: OrderedFloat<f64> = parts
            .next()
            .context("Missing input voltage")?
            .parse()
            .context("Invalid input voltage")?;
        let _ = parts.next().context("Missing value 2")?;
        let output_voltage: OrderedFloat<f64> = parts
            .next()
            .context("Missing output voltage")?
            .parse()
            .context("Invalid output voltage")?;
        let output_load_percent: u8 = parts
            .next()
            .context("Missing output load percent")?
            .parse()
            .context("Invalid output load percent")?;
        let output_frequency: OrderedFloat<f64> = parts
            .next()
            .context("Missing output frequency")?
            .parse()
            .context("Invalid output frequency")?;
        let battery_voltage: OrderedFloat<f64> = parts
            .next()
            .context("Missing battery voltage")?
            .parse()
            .context("Invalid battery voltage")?;

        let _ = parts.next().context("Missing value 7")?;
        let status = parts.next().context("Missing value status")?;

        if status.len() < 8 {
            return Err(anyhow::anyhow!("Status had invalid length"));
        }

        let mut status_bits = ['0'; 8];

        // Collect the status bits
        status_bits
            .iter_mut()
            .zip(status.chars())
            .for_each(|(status_bit, status)| {
                *status_bit = status;
            });

        let status_bit_utility = status_bits[0];
        let status_bit_battery_low = status_bits[1];
        let _status_bit_2 = status_bits[2];
        let status_bit_fault_mode = status_bits[3];
        let status_bit_device_type = status_bits[4];
        let status_bit_self_test_progress = status_bits[5];
        let _status_bit_6 = status_bits[6];
        let status_bit_buzzer_control = status_bits[7];

        let device_power_state = match status_bit_utility {
            '0' => DevicePowerState::Utility,
            '1' => DevicePowerState::Battery,
            _ => return Err(anyhow::anyhow!("Invalid device type status")),
        };

        let battery_low = status_bit_battery_low == '1';
        let fault_mode = status_bit_fault_mode == '1';

        let device_line_type = match status_bit_device_type {
            '0' => DeviceLineType::OnLine,
            '1' => DeviceLineType::LineInteractive,
            _ => return Err(anyhow::anyhow!("Invalid device type status")),
        };

        let battery_self_test = status_bit_self_test_progress == '1';
        let buzzer_control = status_bit_buzzer_control == '1';

        Ok(DeviceState {
            input_voltage,
            output_voltage,
            output_load_percent,
            output_frequency,
            battery_voltage,
            device_power_state,
            battery_low,
            fault_mode,
            device_line_type,
            battery_self_test,
            buzzer_control,
        })
    }
}

/// Command to cancel a battery test
pub struct CancelBatteryTest;

impl IntoDeviceCommand for CancelBatteryTest {
    type Response = ExecuteResponse;

    fn get_command(&self) -> CompactString {
        "CT".into()
    }

    fn invalidate_cache(&self, cache: &mut ResponseCache) {
        // Clear the device state cache
        let query = QueryDeviceState;

        if let Some(cache_key) = query.cache_key() {
            cache.remove(cache_key);
        }
    }
}

/// Runs a 10s battery test
pub struct BatteryTest;

impl IntoDeviceCommand for BatteryTest {
    type Response = ();

    fn get_command(&self) -> CompactString {
        "T".into()
    }

    fn invalidate_cache(&self, cache: &mut ResponseCache) {
        // Clear the device state cache
        let query = QueryDeviceState;

        if let Some(cache_key) = query.cache_key() {
            cache.remove(cache_key);
        }
    }
}

/// Command to trigger a delayed shutdown of the UPS and an
/// automatic reboot after a specified delay
pub struct ScheduleUPSShutdown {
    /// Minutes to wait before shutting down
    pub delay_minutes: f32,

    /// Delay in minutes before turning back on
    pub reboot_delay_minutes: u16,
}

impl IntoDeviceCommand for ScheduleUPSShutdown {
    type Response = ();

    fn get_command(&self) -> CompactString {
        let delay_minutes = self.delay_minutes.min(9999.0);
        let reboot_delay_minutes = self.reboot_delay_minutes.min(9999);

        format_compact!("S{}R{:04}", delay_minutes, reboot_delay_minutes)
    }
}

/// Toggles the buzzer state
/// (A.k.a the beep sound to play when the UPS looses power)
pub struct ToggleBuzzer;

impl IntoDeviceCommand for ToggleBuzzer {
    type Response = ();

    fn get_command(&self) -> CompactString {
        "Q".into()
    }

    fn invalidate_cache(&self, cache: &mut ResponseCache) {
        // Clear the device state cache
        let query = QueryDeviceState;

        if let Some(cache_key) = query.cache_key() {
            cache.remove(cache_key);
        }
    }
}

#[cfg(test)]
mod test {
    use ordered_float::OrderedFloat;

    use crate::ups::{
        command::FromDeviceResponse,
        models::{DeviceBattery, DeviceLineType, DevicePowerState, DeviceState},
    };

    /// Should parse a valid device battery state
    #[test]
    fn test_parse_device_battery() {
        let value = "(100 02832 50.0 000.5 175 290 0 0000020000112000";
        let battery = DeviceBattery::from_device_response(value.into())
            .expect("Battery should parse successfully");
        let expected = DeviceBattery {
            capacity: 100,
            remaining_time: 2832,
        };
        assert_eq!(battery.capacity, expected.capacity);
        assert_eq!(battery.remaining_time, expected.remaining_time);
    }

    /// Should fail on a malformed response
    #[test]
    fn test_fail_parse_device_battery() {
        let value = "(A B 50.0 000.5 175 290 0 0000020000112000";
        DeviceBattery::from_device_response(value.into()).expect_err("Battery should fail parsing");
    }

    /// Should parse a valid device state
    #[test]
    fn test_parse_device_state() {
        let value = "(237.1 237.1 237.1 008 50.1 27.1 --.- 00001001";
        let battery = DeviceState::from_device_response(value.into())
            .expect("Battery should parse successfully");
        let expected = DeviceState {
            input_voltage: OrderedFloat(237.1),
            output_voltage: OrderedFloat(237.1),
            output_load_percent: 8,
            output_frequency: OrderedFloat(50.1),
            battery_voltage: OrderedFloat(27.1),
            device_power_state: DevicePowerState::Utility,
            battery_low: false,
            fault_mode: false,
            device_line_type: DeviceLineType::LineInteractive,
            battery_self_test: false,
            buzzer_control: true,
        };
        assert_eq!(battery.input_voltage, expected.input_voltage);
        assert_eq!(battery.output_voltage, expected.output_voltage);
        assert_eq!(battery.output_load_percent, expected.output_load_percent);
        assert_eq!(battery.output_frequency, expected.output_frequency);
        assert_eq!(battery.battery_voltage, expected.battery_voltage);
        assert_eq!(battery.device_power_state, expected.device_power_state);
        assert_eq!(battery.battery_low, expected.battery_low);
        assert_eq!(battery.fault_mode, expected.fault_mode);
        assert_eq!(battery.device_line_type, expected.device_line_type);
        assert_eq!(battery.battery_self_test, expected.battery_self_test);
        assert_eq!(battery.buzzer_control, expected.buzzer_control);
    }

    /// Should fail on a malformed response
    #[test]
    fn test_fail_parse_device_state() {
        let value = "(A B 237.1 008 50.1 27.1 --.- 00001001";
        DeviceState::from_device_response(value.into()).expect_err("Battery should fail parsing");
    }
}
