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

    /// Pushes a new event to any of the watchers
    pub fn push_event(&mut self, event: UPSEvent) {
        _ = self.tx.send(event);
    }

    /// Handle polling the device state at the expected interval
    /// and checking any changes
    pub async fn process(mut self) {
        while self.tx.receiver_count() > 0 && self.executor.is_open() {
            self.process_device_state().await;

            sleep(POLL_INTERVAL).await;
        }
    }

    /// Requests the current device state from the executor and checks
    /// for any state changes, emits events for changed states
    pub async fn process_device_state(&mut self) {
        let device_state = match self.executor.request(QueryDeviceState).await {
            Ok(value) => value,
            Err(err) => {
                error!("Error while requesting UPS device state: {err:?}");
                return;
            }
        };

        // Obtain the previous states if available
        let (last_battery_self_test, last_battery_low, last_device_power_state) = self
            .last_device_state
            .as_ref()
            .map(|value| {
                (
                    Some(value.battery_self_test),
                    Some(value.battery_low),
                    Some(value.device_power_state),
                )
            })
            .unwrap_or_default();

        // Battery self tests
        match (last_battery_self_test, device_state.battery_self_test) {
            // Should trigger enter event if there is a transition or none previous state
            (Some(false) | None, true) => {
                info!("Device has started self test");

                self.push_event(UPSEvent::BatteryTestStart);
            }
            (Some(true), false) => {
                info!("Device has finished self test");

                self.push_event(UPSEvent::BatteryTestEnd);
            }
            _ => {}
        }

        // Low battery
        match (last_battery_low, device_state.battery_low) {
            // Should trigger enter event if there is a transition or none previous state
            (Some(false) | None, true) => {
                info!("Device is running low on battery");

                self.push_event(UPSEvent::LowBatteryModeStart);
            }
            (Some(true), false) => {
                info!("Device is no longer low on battery");
                self.push_event(UPSEvent::LowBatteryModeEnd);
            }
            _ => {}
        }

        // Power transitions
        match (last_device_power_state, device_state.device_power_state) {
            (Some(DevicePowerState::Utility) | None, DevicePowerState::Battery) => {
                warn!("AC FAILURE");

                self.push_event(UPSEvent::ACFailure);
            }
            (Some(DevicePowerState::Battery), DevicePowerState::Utility) => {
                info!("AC RECOVERY");

                self.push_event(UPSEvent::ACRecovery);
            }
            _ => {}
        };

        self.last_device_state = Some(device_state);
    }
}
