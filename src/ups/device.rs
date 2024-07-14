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
    use std::sync::Arc;

    use std::sync::Mutex;

    use super::Device;
    use super::DeviceCreator;

    /// Mocking responder implementation that lets you set
    /// the next response the [MockDevice] will respond with
    #[derive(Default, Clone)]
    pub struct MockDeviceResponder {
        /// Next response to use
        next: Arc<Mutex<String>>,
    }

    impl MockDeviceResponder {
        /// Get the next response
        pub fn next_response(&self) -> String {
            self.next.lock().unwrap().clone()
        }

        /// Set the next response
        pub fn set_next_response(&self, next: String) {
            *self.next.lock().unwrap() = next;
        }
    }

    /// Creator for creating [MockDevice]s that will respond using
    /// the provided shared responder
    pub struct MockDeviceCreator {
        responder: MockDeviceResponder,
    }

    impl MockDeviceCreator {
        /// Create a new creator, provides the creator itself and the responder handle
        pub fn new() -> (Self, MockDeviceResponder) {
            let responder: MockDeviceResponder = Default::default();
            let creator = Self {
                responder: responder.clone(),
            };

            (creator, responder)
        }
    }

    impl DeviceCreator for MockDeviceCreator {
        type Output = MockDevice;
        fn try_create_device(&self) -> anyhow::Result<Self::Output> {
            Ok(MockDevice {
                responder: self.responder.clone(),
            })
        }
    }

    pub struct MockDevice {
        responder: MockDeviceResponder,
    }

    impl Device for MockDevice {
        type Creator = MockDeviceCreator;

        fn read_response(&mut self) -> anyhow::Result<String> {
            Ok(self.responder.next_response())
        }

        fn write_command(&mut self, _cmd: &str) -> anyhow::Result<()> {
            Ok(())
        }
    }
}
