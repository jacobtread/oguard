use std::time::Duration;

use log::{error, info, warn};
use tokio::{sync::broadcast, time::sleep};

use crate::ups::{DevicePowerState, DeviceState, UPSExecutorHandle};
/// Interval between each device state poll
const POLL_INTERVAL: Duration = Duration::from_secs(1);

/// Watcher that polls a UPS executor at fixed intervals
/// to handle changes in the state
pub struct UPSWatcher {
    executor: UPSExecutorHandle,
    tx: broadcast::Sender<UPSWatcherMessage>,
    last_device_state: Option<DeviceState>,
}

/// Handle to a [UPSWatcher] to receive messages/events
pub struct UPSWatcherHandle {
    rx: broadcast::Receiver<UPSWatcherMessage>,
}

impl Clone for UPSWatcherHandle {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.resubscribe(),
        }
    }
}

impl UPSWatcherHandle {
    /// Receive the next watcher message
    pub async fn next(&mut self) -> Option<UPSWatcherMessage> {
        self.rx.recv().await.ok()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UPSWatcherMessage {
    Event(UPSEvent),
}

/// Events that could be encountered while processing state updates
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UPSEvent {
    /// AC Power has been lost
    ACFailure,
    /// AC Power has been recovered
    ACRecovery,
    /// UPS has encountered a fault
    UPSFault,
    /// UPS has low battery
    LowBatteryMode,
}

impl UPSWatcher {
    pub fn start(executor: UPSExecutorHandle) -> UPSWatcherHandle {
        let (tx, rx) = broadcast::channel(4);
        let watcher = Self {
            executor,
            last_device_state: None,
            tx,
        };
        tokio::spawn(watcher.process());

        UPSWatcherHandle { rx }
    }

    pub async fn process(mut self) {
        while self.tx.receiver_count() > 0 && self.executor.is_open() {
            let device_state = match self.executor.device_state().await {
                Ok(value) => value,
                Err(err) => {
                    error!("Error while requesting UPS state: {err:?}");
                    return;
                }
            };

            let Some(last_state) = self.last_device_state.as_ref() else {
                self.last_device_state = Some(device_state);
                continue;
            };

            // Self test has finished
            if last_state.battery_self_test && !device_state.battery_self_test {
                // END SELF TEST EVENT
            }

            // Entered low battery mode
            if !last_state.battery_low && device_state.battery_low {
                info!("Device has entered low power mode");

                _ = self
                    .tx
                    .send(UPSWatcherMessage::Event(UPSEvent::LowBatteryMode));
            }

            match (
                device_state.device_power_state,
                last_state.device_power_state,
            ) {
                (DevicePowerState::Utility, DevicePowerState::Battery) => {
                    info!("AC RECOVERY");

                    _ = self.tx.send(UPSWatcherMessage::Event(UPSEvent::ACRecovery));
                }
                (DevicePowerState::Battery, DevicePowerState::Utility) => {
                    warn!("AC FAILURE");

                    _ = self.tx.send(UPSWatcherMessage::Event(UPSEvent::ACFailure));
                }

                _ => {
                    // No power event has occurred
                }
            };

            self.last_device_state = Some(device_state);

            sleep(POLL_INTERVAL).await;
        }
    }
}
