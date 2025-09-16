//! # Device Executor
//!
//! Executor implementation for accessing a UPS device, allows a generic underlying
//! implementation so that testing can use a mock implementation.
//!
//! Provides a handle that allows the device to have multiple commands sent in an
//! asynchronous manner without attempting to write multiple at the same time.
//!
//! The HID device can only write and read a single request and response at a time so
//! the device executor will process each request one-by-one ensuring multiple requests
//! don't corrupt the previous request

use super::{
    command::{FromDeviceResponse, IntoDeviceCommand, ResponseCache},
    device::{DefaultDevice, Device, DeviceCreator},
};
use anyhow::{Context, anyhow};
use hidapi::HidError;
use log::{error, info, warn};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

/// Dynamic device command with the response type erased
type DeviceCommandDyn<D> = Box<dyn DeviceCommandProxy<D>>;

/// Request for a device to execute, contains the channel to
/// send back the response
pub struct DeviceCommand<R: FromDeviceResponse> {
    /// The command to execute
    command: Box<dyn IntoDeviceCommand<Response = R>>,
    /// Channel to send the response to
    tx: Option<oneshot::Sender<anyhow::Result<R>>>,
}

/// Executor that can run commands on a device
pub struct DeviceExecutor<D: Device = DefaultDevice> {
    /// Device to execute commands on
    device: D,

    /// Device creator to make new device connections if
    /// the existing connection fails
    device_creator: D::Creator,

    /// Cache for responses
    cache: ResponseCache,

    /// Channel to receive commands to execute
    rx: mpsc::Receiver<DeviceCommandDyn<D>>,
}

impl<D: Device> DeviceExecutor<D> {
    /// Duration to wait before attempting to retry creating a device
    const RETRY_CREATE_DURATION: Duration = Duration::from_secs(5);

    pub fn start(device_creator: D::Creator) -> anyhow::Result<DeviceExecutorHandle<D>> {
        // Channel for receiving commands
        let (tx, rx) = mpsc::channel(8);

        let cache = ResponseCache::default();

        let device = Self::try_create_device(&device_creator).context("create device")?;

        let executor = DeviceExecutor {
            device,
            device_creator,
            rx,
            cache,
        };

        std::thread::spawn(move || executor.process());

        Ok(DeviceExecutorHandle { tx })
    }

    /// Attempts to connect to the HID device
    fn try_create_device(device_creator: &D::Creator) -> anyhow::Result<D> {
        let max_attempts = 5;
        let mut attempt = 0;

        let mut last_error: Option<anyhow::Error> = None;

        while attempt < max_attempts {
            match device_creator.try_create_device() {
                Ok(device) => return Ok(device),
                Err(err) => {
                    warn!(
                        "failed to create device handle, retrying in {} seconds: {}",
                        Self::RETRY_CREATE_DURATION.as_secs(),
                        err
                    );
                    last_error = Some(err);
                }
            }

            std::thread::sleep(Self::RETRY_CREATE_DURATION);

            attempt += 1;
        }

        let last_error = last_error.expect("missing last error");

        Err(anyhow!("failed to acquire device: {}", last_error))
    }

    fn process(mut self) {
        while let Some(mut msg) = self.rx.blocking_recv() {
            while let HandleOutcome::Disconnected = msg.handle(&mut self.device, &mut self.cache) {
                // Handle is closed, try and create a new one
                warn!("lost device handle, attempting to create a new one");

                let device = match Self::try_create_device(&self.device_creator) {
                    Ok(device) => device,
                    Err(err) => {
                        error!("failed to acquire new handle: {err}");
                        return;
                    }
                };

                info!("acquired new device handle, resuming");

                // Swap device handle
                self.device = device;
            }
        }
    }
}

/// Handle to a [DeviceExecutor] for sending commands
pub struct DeviceExecutorHandle<D: Device = DefaultDevice> {
    /// Channel to send commands
    tx: mpsc::Sender<DeviceCommandDyn<D>>,
}

impl<D: Device> Clone for DeviceExecutorHandle<D> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<D: Device> DeviceExecutorHandle<D> {
    /// Checks whether the underlying channel is still open
    pub fn is_open(&self) -> bool {
        !self.tx.is_closed()
    }

    /// Sends a command to the device and receives the response
    pub async fn send<C>(&self, command: C) -> anyhow::Result<C::Response>
    where
        C: IntoDeviceCommand,
    {
        let (tx, rx) = oneshot::channel();

        let msg = DeviceCommand {
            command: Box::new(command),
            tx: Some(tx),
        };

        self.tx
            .send(Box::new(msg))
            .await
            .map_err(|_| anyhow!("device channel closed"))?;

        let value = rx.await.context("failed to get command response")??;
        Ok(value)
    }
}

/// Type erased proxy over [DeviceCommand] to allow them to
/// be handled and send their response in a dynamic context
trait DeviceCommandProxy<D: Device>: Send + 'static {
    /// Handle the request
    fn handle(&mut self, device: &mut D, cache: &mut ResponseCache) -> HandleOutcome;
}

enum HandleOutcome {
    /// Continue handling next item
    Continue,
    /// Device was disconnected
    Disconnected,
}

impl<D: Device, R: FromDeviceResponse> DeviceCommandProxy<D> for DeviceCommand<R> {
    fn handle(&mut self, device: &mut D, cache: &mut ResponseCache) -> HandleOutcome {
        let cache_key = self.command.cache_key();
        if let Some(cached_response) = cache.get(cache_key) {
            // Send the cached response
            if let Some(tx) = self.tx.take() {
                _ = tx.send(Ok(cached_response));
            }
        }

        // Get and execute the command
        let command = self.command.get_command();
        let result = device
            // Send the command
            .send_command(&command)
            // Attempt to parse the response
            .and_then(R::from_device_response);

        if is_disconnect_error(result.as_ref()) {
            return HandleOutcome::Disconnected;
        }

        // Store successful responses
        if let Some(cache_key) = cache_key
            && let Ok(value) = result.as_ref() {
                cache.insert(cache_key, value);
            }

        // Invalidate cache keys
        self.command.invalidate_cache(cache);

        // Send the response
        if let Some(tx) = self.tx.take() {
            _ = tx.send(result);
        }

        HandleOutcome::Continue
    }
}

/// Checks if the provided result contains a disconnect error
fn is_disconnect_error<T>(result: Result<&T, &anyhow::Error>) -> bool {
    result
        .err()
        // Try downcast to a HID error
        .and_then(|err| err.downcast_ref::<HidError>())
        // Check if the error contains the disconnected message
        .is_some_and(|value| value.to_string().contains("The device is not connected"))
}
