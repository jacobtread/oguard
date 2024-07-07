use std::{
    any::Any,
    collections::HashMap,
    time::{Duration, Instant},
};

use anyhow::Context;
use hidapi::{HidApi, HidDevice};
use moka::{policy::EvictionPolicy, sync::Cache};
use ordered_float::OrderedFloat;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, oneshot},
    time::timeout,
};

/// HID Device Vendor ID
const VENDOR_ID: u16 = 0x0665;
/// HID Device Product ID
const PRODUCT_ID: u16 = 0x5161;

/// Dynamic requests
type UPSRequest = Box<dyn DeviceRequestProxy>;

#[derive(Default)]
pub struct ResponseCache {
    pub cache: HashMap<u64, CachedValue>,
}

pub struct CachedValue {
    pub value: Box<dyn Any + Send>,
    pub expires_at: Instant,
}

/// Executor for running the synchronous requests over the USB HID
pub struct UPSExecutor {
    device: HidDevice,
    cache: ResponseCache,
    rx: mpsc::Receiver<UPSRequest>,
}

impl UPSExecutor {
    pub fn start() -> anyhow::Result<UPSExecutorHandle> {
        let (tx, rx) = mpsc::channel(8);

        let api = HidApi::new().context("Failed to create HID API")?;

        let device = api
            .open(VENDOR_ID, PRODUCT_ID)
            .expect("Failed to open device");

        let cache = ResponseCache::default();

        let executor = UPSExecutor { device, rx, cache };

        std::thread::spawn(move || executor.process());

        Ok(UPSExecutorHandle { tx })
    }

    pub fn process(mut self) {
        while let Some(mut msg) = self.rx.blocking_recv() {
            msg.handle(&mut self.device, &mut self.cache);
        }
    }
}

#[derive(Clone)]
pub struct UPSExecutorHandle {
    tx: mpsc::Sender<UPSRequest>,
}

impl UPSExecutorHandle {
    pub fn is_open(&self) -> bool {
        !self.tx.is_closed()
    }

    pub async fn request<C>(&self, command: C) -> anyhow::Result<C::Response>
    where
        C: DeviceCommand,
    {
        let (tx, rx) = oneshot::channel();

        let msg = DeviceRequest {
            command: Box::new(command),
            tx: Some(tx),
        };

        if self.tx.send(Box::new(msg)).await.is_err() {
            return Err(anyhow::anyhow!("device channel closed"));
        }

        let value = rx.await.context("Failed to recv device state")??;
        Ok(value)
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

/// Sends a command over the device HID, commands begin with the report ID which
/// is always zero and end with a carriage return to indicate the end of a command
fn execute_command(device: &mut HidDevice, cmd: &str) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    buffer.push(0); // Report ID
    buffer.extend_from_slice(cmd.as_bytes());
    buffer.push(b'\r');
    device.write(&buffer)?;
    Ok(())
}

/// Reads a response from the device
fn read_response(device: &mut HidDevice) -> anyhow::Result<String> {
    let mut out = String::new();

    let mut buffer = [0u8; 128];

    loop {
        let count = device
            .read_timeout(&mut buffer, 3000)
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

/// Request for a device to execute
pub struct DeviceRequest<T> {
    /// The command to execute
    command: Box<dyn DeviceCommand<Response = T>>,
    /// Channel to send the response to
    tx: Option<oneshot::Sender<anyhow::Result<T>>>,
}

/// Trait for commands that can be executed
pub trait DeviceCommand: Send + 'static {
    /// The device response
    type Response: Send + Clone + 'static;

    /// Executes the command on the device returning the response
    fn execute(&mut self, device: &mut HidDevice) -> anyhow::Result<Self::Response>;

    /// Unique cache key to use if the response can be cached
    fn cache_key(&self) -> Option<u64> {
        None
    }
}

/// Type erased proxy over [DeviceRequest] to allow them to
/// be handled and send their response in a dynamic context
pub trait DeviceRequestProxy: Send + 'static {
    /// Handle the request
    fn handle(&mut self, device: &mut HidDevice, cache: &mut ResponseCache);
}

fn get_cached_response<T>(cache_key: Option<u64>, cache: &ResponseCache) -> Option<T>
where
    T: Send + Clone + 'static,
{
    let cache_key = cache_key?;
    let cache_value = cache.cache.get(&cache_key)?;

    if cache_value.expires_at <= Instant::now() {
        return None;
    }

    let value = cache_value.value.downcast_ref::<T>();
    value.cloned()
}

impl<T: Send + Clone + 'static> DeviceRequestProxy for DeviceRequest<T> {
    fn handle(&mut self, device: &mut HidDevice, cache: &mut ResponseCache) {
        let cache_key = self.command.cache_key();
        if let Some(cached_response) = get_cached_response(cache_key, cache) {
            // Send the cached response
            if let Some(tx) = self.tx.take() {
                _ = tx.send(Ok(cached_response));
            }
        }

        // Execute the command
        let result = self.command.execute(device);

        // Store successful responses
        if let Some(cache_key) = cache_key {
            if let Ok(value) = result.as_ref() {
                let value = value.clone();
                let cache_value = CachedValue {
                    expires_at: Instant::now() + Duration::from_secs(1),
                    value: Box::new(value),
                };

                cache.cache.insert(cache_key, cache_value);
            }
        }

        // Send the response
        if let Some(tx) = self.tx.take() {
            _ = tx.send(result);
        }
    }
}

/// Query command to load the device battery
pub struct QueryDeviceBattery;

impl DeviceCommand for QueryDeviceBattery {
    type Response = DeviceBattery;

    fn execute(&mut self, device: &mut HidDevice) -> anyhow::Result<Self::Response> {
        execute_command(device, "QI").context("write request")?;
        let response = read_response(device).context("read response")?;
        parse_device_battery(&response).context("parse response")
    }

    fn cache_key(&self) -> Option<u64> {
        Some(0)
    }
}

/// Parses a device battery response
fn parse_device_battery(msg: &str) -> anyhow::Result<DeviceBattery> {
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

/// Query command to load the device state
pub struct QueryDeviceState;

impl DeviceCommand for QueryDeviceState {
    type Response = DeviceState;

    fn execute(&mut self, device: &mut HidDevice) -> anyhow::Result<Self::Response> {
        execute_command(device, "QS").context("write request")?;
        let response = read_response(device).context("read response")?;
        parse_device_state(&response).context("parse response")
    }

    fn cache_key(&self) -> Option<u64> {
        Some(1)
    }
}

/// Parses the device state response
fn parse_device_state(msg: &str) -> anyhow::Result<DeviceState> {
    // (237.1 237.1 237.1 008 50.1 27.1 --.- 00001001

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

/// Command to cancel a battery test
pub struct CancelBatteryTest;

impl DeviceCommand for CancelBatteryTest {
    type Response = ExecuteResponse;

    fn execute(&mut self, device: &mut HidDevice) -> anyhow::Result<Self::Response> {
        execute_command(device, "CT").context("write request")?;
        let response = read_response(device).context("read response")?;
        parse_execute_response(&response).context("parse response")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExecuteResponse {
    Success,
    Failure,
}

/// Parses a device battery response
fn parse_execute_response(msg: &str) -> anyhow::Result<ExecuteResponse> {
    // 100 02832 50.0 000.5 175 290 0 0000020000112000
    let msg: &str = msg
        .strip_prefix('(')
        .context("Missing execute response prefix")?;

    if msg == "ACK" {
        return Ok(ExecuteResponse::Success);
    }

    Ok(ExecuteResponse::Failure)
}

/// Runs a 10s battery test
pub struct BatteryTest;

impl DeviceCommand for BatteryTest {
    type Response = ();

    fn execute(&mut self, device: &mut HidDevice) -> anyhow::Result<Self::Response> {
        execute_command(device, "T").context("write request")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use anyhow::Context;
    use hidapi::HidDevice;
    use ordered_float::OrderedFloat;
    use tokio::time::sleep;

    use crate::ups::{
        execute_command, BatteryTest, DeviceCommand, DeviceLineType, DevicePowerState, DeviceState,
        QueryDeviceState, UPSExecutor,
    };

    use super::{parse_device_battery, parse_device_state, DeviceBattery};

    /// Should parse a valid device battery state
    #[test]
    fn test_parse_device_battery() {
        let value = "(100 02832 50.0 000.5 175 290 0 0000020000112000";
        let battery = parse_device_battery(value).expect("Battery should parse successfully");
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
        parse_device_battery(value).expect_err("Battery should fail parsing");
    }

    /// Should parse a valid device state
    #[test]
    fn test_parse_device_state() {
        let value = "(237.1 237.1 237.1 008 50.1 27.1 --.- 00001001";
        let battery = parse_device_state(value).expect("Battery should parse successfully");
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
        parse_device_state(value).expect_err("Battery should fail parsing");
    }

    /// Run battery self test
    #[tokio::test]
    #[ignore]
    async fn test_battery_test() {
        // Start the executor
        let executor = UPSExecutor::start().unwrap();
        executor.request(BatteryTest).await.unwrap();

        sleep(Duration::from_secs(1)).await;

        let device_state = executor.request(QueryDeviceState).await.unwrap();
        assert!(device_state.battery_self_test);

        sleep(Duration::from_secs(11)).await;

        let device_state = executor.request(QueryDeviceState).await.unwrap();
        assert!(!device_state.battery_self_test);
    }

    #[tokio::test]
    #[ignore]
    async fn test_new_query() {
        dotenvy::dotenv().unwrap();
        env_logger::init();

        // Start the executor
        let executor = UPSExecutor::start().unwrap();

        struct TestQuery;

        impl DeviceCommand for TestQuery {
            type Response = ();

            fn execute(&mut self, device: &mut HidDevice) -> anyhow::Result<Self::Response> {
                execute_command(device, "").context("write device state request")?;

                Ok(())
            }
        }

        executor.request(TestQuery).await.unwrap();
    }
}
