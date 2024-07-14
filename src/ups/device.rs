use anyhow::Context;
use hidapi::{HidApi, HidDevice};

/// Creator for devices
pub trait DeviceCreator: Sized + Send + 'static {
    type Output: Device;

    fn try_create_device(&self) -> anyhow::Result<Self::Output>;
}

/// Trait implemented by a device that can execute commands
pub trait Device: Sized + Send + 'static {
    type Creator: DeviceCreator<Output = Self>;

    /// Sends a command to the device and receives the response
    fn send_command(&mut self, cmd: &str) -> anyhow::Result<String> {
        self.write_command(cmd).context("write command")?;
        self.read_response().context("read response")
    }

    /// Writes a command to the device
    fn write_command(&mut self, cmd: &str) -> anyhow::Result<()>;

    /// Reads a string response from the device
    fn read_response(&mut self) -> anyhow::Result<String>;
}

pub struct HidDeviceCreator {
    /// API to create the device with
    api: HidApi,
}

impl HidDeviceCreator {
    pub fn new() -> anyhow::Result<Self> {
        let api = HidApi::new().context("failed to create hid api")?;

        Ok(Self { api })
    }
}

impl DeviceCreator for HidDeviceCreator {
    type Output = HidDevice;

    fn try_create_device(&self) -> anyhow::Result<Self::Output> {
        let device = self
            .api
            .open(VENDOR_ID, PRODUCT_ID)
            .context("failed to open device")?;

        Ok(device)
    }
}

/// HID Device Vendor ID
const VENDOR_ID: u16 = 0x0665;
/// HID Device Product ID
const PRODUCT_ID: u16 = 0x5161;

impl Device for HidDevice {
    type Creator = HidDeviceCreator;

    /// Sends a command over the device HID, commands begin with the report ID which
    /// is always zero and end with a carriage return to indicate the end of a command
    fn write_command(&mut self, cmd: &str) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        buffer.push(0); // Report ID
        buffer.extend_from_slice(cmd.as_bytes());
        buffer.push(b'\r');
        self.write(&buffer)?;
        Ok(())
    }

    /// Reads a response from the device, terminated by a carriage return
    /// or 3s timeout
    fn read_response(&mut self) -> anyhow::Result<String> {
        let mut out = String::new();

        let mut buffer = [0u8; 128];

        loop {
            let count = self
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
}

#[cfg(test)]
pub mod test {
    use anyhow::Context;
    use tokio::sync::broadcast;
    use tokio::sync::mpsc;

    use super::Device;
    use super::DeviceCreator;

    /// Mocking responder implementation that lets you set
    /// the next response the [MockDevice] will respond with
    pub struct MockDeviceHandle {
        /// Sender for the next response
        tx: broadcast::Sender<String>,
        /// Receiver for the next command
        rx: mpsc::Receiver<String>,
    }

    impl MockDeviceHandle {
        /// Set the next response
        pub async fn next_response(&self, next: String) {
            _ = self.tx.send(next);
        }

        /// Await the next executed command
        pub async fn next_command(&mut self) -> Option<String> {
            self.rx.recv().await
        }
    }

    /// Creator for creating [MockDevice]s that will respond using
    /// the provided shared responder
    pub struct MockDeviceCreator {
        /// Sender for command messages
        tx: mpsc::Sender<String>,
        /// Receiver for responses
        rx: broadcast::Receiver<String>,
    }

    impl MockDeviceCreator {
        /// Create a new creator, provides the creator itself and the responder handle
        pub fn new() -> (Self, MockDeviceHandle) {
            let (tx_cmd, rx_cmd) = mpsc::channel(8);
            let (tx_res, rx_res) = broadcast::channel(8);
            let handle: MockDeviceHandle = MockDeviceHandle {
                tx: tx_res,
                rx: rx_cmd,
            };
            let creator = Self {
                tx: tx_cmd,
                rx: rx_res,
            };

            (creator, handle)
        }
    }

    impl DeviceCreator for MockDeviceCreator {
        type Output = MockDevice;
        fn try_create_device(&self) -> anyhow::Result<Self::Output> {
            Ok(MockDevice {
                tx: self.tx.clone(),
                rx: self.rx.resubscribe(),
            })
        }
    }

    pub struct MockDevice {
        /// Sender for command messages
        tx: mpsc::Sender<String>,
        /// Receiver for responses
        rx: broadcast::Receiver<String>,
    }

    impl Device for MockDevice {
        type Creator = MockDeviceCreator;

        fn read_response(&mut self) -> anyhow::Result<String> {
            self.rx.blocking_recv().context("missing response")
        }

        fn write_command(&mut self, cmd: &str) -> anyhow::Result<()> {
            _ = self.tx.send(cmd.to_string());
            Ok(())
        }
    }
}
