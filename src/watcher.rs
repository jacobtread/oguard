use crate::ups::{DevicePowerState, DeviceState, QueryDeviceState, UPSExecutorHandle};
use log::{error, info, warn};
use serde::Serialize;
use std::time::Duration;
use tokio::{sync::broadcast, time::sleep};
use tokio_stream::wrappers::BroadcastStream;

/// Interval between each device state poll
const POLL_INTERVAL: Duration = Duration::from_secs(3);

/// Watcher that polls a UPS executor at fixed intervals
/// to handle changes in the state
pub struct UPSWatcher {
    executor: UPSExecutorHandle,
    tx: broadcast::Sender<UPSEvent>,
    last_device_state: Option<DeviceState>,
}

/// Handle to a [UPSWatcher] to receive messages/events
pub struct UPSWatcherHandle {
    rx: broadcast::Receiver<UPSEvent>,
}

impl Clone for UPSWatcherHandle {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.resubscribe(),
        }
    }
}

impl UPSWatcherHandle {
    pub fn into_stream(self) -> BroadcastStream<UPSEvent> {
        BroadcastStream::new(self.rx)
    }

    /// Receive the next watcher message
    pub async fn next(&mut self) -> Option<UPSEvent> {
        self.rx.recv().await.ok()
    }
}

/// Events that could be encountered while processing state updates
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[serde(tag = "type")]
pub enum UPSEvent {
    /// AC Power has been lost
    ACFailure,
    /// AC Power has been recovered
    ACRecovery,
    /// UPS has encountered a fault
    UPSFault,
    /// UPS has entered low battery mode
    LowBatteryModeStart,
    /// UPS has left low power mode
    LowBatteryModeEnd,
    /// UPS Battery test has started
    BatteryTestStart,
    /// UPS Battery test has ended
    BatteryTestEnd,
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
            let device_state = match self.executor.request(QueryDeviceState).await {
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

            // Battery self tests
            match (last_state.battery_self_test, device_state.battery_self_test) {
                (false, true) => {
                    info!("Device has started self test");

                    _ = self.tx.send(UPSEvent::BatteryTestStart);
                }
                (true, false) => {
                    info!("Device has finished self test");

                    _ = self.tx.send(UPSEvent::BatteryTestEnd);
                }
                _ => {}
            }

            // Low battery
            match (last_state.battery_low, device_state.battery_low) {
                (false, true) => {
                    info!("Device is running low on battery");

                    _ = self.tx.send(UPSEvent::LowBatteryModeStart);
                }
                (true, false) => {
                    info!("Device is no longer low on battery");

                    _ = self.tx.send(UPSEvent::LowBatteryModeEnd);
                }
                _ => {}
            }

            // Power transitions
            match (
                device_state.device_power_state,
                last_state.device_power_state,
            ) {
                (DevicePowerState::Utility, DevicePowerState::Battery) => {
                    info!("AC RECOVERY");

                    _ = self.tx.send(UPSEvent::ACRecovery);
                }
                (DevicePowerState::Battery, DevicePowerState::Utility) => {
                    warn!("AC FAILURE");

                    _ = self.tx.send(UPSEvent::ACFailure);
                }
                _ => {}
            };

            self.last_device_state = Some(device_state);

            sleep(POLL_INTERVAL).await;
        }
    }
}
