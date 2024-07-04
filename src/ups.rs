use anyhow::Context;
use hidapi::{HidApi, HidDevice};
use tokio::sync::{mpsc, oneshot};

/// HID Device Vendor ID
const VENDOR_ID: u16 = 0x0665;
/// HID Device Product ID
const PRODUCT_ID: u16 = 0x5161;

/// Requests for the UPS executor to fulfill
#[derive(Debug)]
pub enum UPSRequest {
    DeviceState(oneshot::Sender<anyhow::Result<DeviceState>>),
    DeviceBattery(oneshot::Sender<anyhow::Result<DeviceBattery>>),
}

/// Executor for running the synchronous requests over the USB HID
pub struct UPSExecutor {
    device: HidDevice,
    rx: mpsc::Receiver<UPSRequest>,
}

impl UPSExecutor {
    pub fn start() -> anyhow::Result<UPSExecutorHandle> {
        let (tx, rx) = mpsc::channel(8);

        let api = HidApi::new().context("Failed to create HID API")?;

        let device = api
            .open(VENDOR_ID, PRODUCT_ID)
            .expect("Failed to open device");

        let executor = UPSExecutor { device, rx };

        std::thread::spawn(move || executor.process());

        Ok(UPSExecutorHandle { tx })
    }

    pub fn process(mut self) {
        while let Some(msg) = self.rx.blocking_recv() {
            match msg {
                UPSRequest::DeviceState(tx) => {
                    let result = query_device_state(&mut self.device);
                    _ = tx.send(result);
                }
                UPSRequest::DeviceBattery(tx) => {
                    let result = query_device_battery(&mut self.device);
                    _ = tx.send(result);
                }
            }
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

    /// Request the device battery details
    pub async fn device_state(&self) -> anyhow::Result<DeviceState> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(UPSRequest::DeviceState(tx))
            .await
            .context("UPS device channel was closed")?;
        let value = rx.await.context("Failed to recv device state")??;
        Ok(value)
    }

    /// Request the device battery details
    pub async fn device_battery(&self) -> anyhow::Result<DeviceBattery> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(UPSRequest::DeviceBattery(tx))
            .await
            .context("UPS device channel was closed")?;
        let value = rx.await.context("Failed to recv device battery")??;
        Ok(value)
    }
}

/// UPS device line type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeviceLineType {
    /// Device is line interactive
    LineInteractive,
    /// Device is on-line
    OnLine,
}

/// Current source of power for the UPS
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DevicePowerState {
    /// Device is being powered from a socket, battery is not used
    Utility,
    /// No power coming from socket, battery is in use
    Battery,
}

/// Response from a device battery query
#[derive(Debug, Clone)]
pub struct DeviceBattery {
    /// Capacity of the battery as a percentage 0-100
    pub capacity: u8,
    /// Remaining time of the battery charge in seconds
    pub remaining_time: u32,
}

/// "Work mode" - Possible current states the UPS is in
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(Debug)]
pub struct DeviceState {
    /// Voltage going into the UPS (Power coming from wall)
    pub input_voltage: f64,
    /// Voltage coming out of the UPS (Power coming from UPS)
    pub output_voltage: f64,
    /// Percentage load/usage of the UPS
    pub output_load_percent: u8,
    /// Output frequency from the UPS
    pub output_frequency: f64,
    /// Voltage of the battery
    pub battery_voltage: f64,
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

        if self.output_voltage < 20.0 {
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

/// Queries the device for its current battery state
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

/// Queries the current state of the UPS device
fn query_device_state(device: &mut HidDevice) -> anyhow::Result<DeviceState> {
    // (237.1 237.1 237.1 008 50.1 27.1 --.- 00001001

    execute_command(device, "QS")?;

    let response = read_response(device)?;
    let response = response
        .strip_prefix('(')
        .context("Missing device battery response prefix")?;

    let mut parts = response.split(' ');

    let input_voltage: f64 = parts
        .next()
        .context("Missing input voltage")?
        .parse()
        .context("Invalid input voltage")?;
    let _ = parts.next().context("Missing value 2")?;
    let output_voltage: f64 = parts
        .next()
        .context("Missing output voltage")?
        .parse()
        .context("Invalid output voltage")?;
    let output_load_percent: u8 = parts
        .next()
        .context("Missing output load percent")?
        .parse()
        .context("Invalid output load percent")?;
    let output_frequency: f64 = parts
        .next()
        .context("Missing output frequency")?
        .parse()
        .context("Invalid output frequency")?;
    let battery_voltage: f64 = parts
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
