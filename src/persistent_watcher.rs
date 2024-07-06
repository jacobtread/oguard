use crate::{
    database::entities::{battery_history::BatteryHistoryModel, state_history::StateHistoryModel},
    ups::{DeviceBattery, DeviceState, QueryDeviceBattery, QueryDeviceState, UPSExecutorHandle},
};
use chrono::Utc;
use log::{debug, error};
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tokio::{
    task::JoinHandle,
    time::{interval_at, Instant, MissedTickBehavior},
};

/// Interval between each device state poll
const POLL_INTERVAL: Duration = Duration::from_secs(60);

/// Watcher that polls a UPS executor at fixed intervals logging the state
/// to the database
pub struct UPSPersistentWatcher {
    /// Executor to execute the requests
    executor: UPSExecutorHandle,
    /// Database connection to store the data
    db: DatabaseConnection,
    /// Last state response
    last_device_state: Option<DeviceState>,
    /// Last battery state response
    last_battery_state: Option<DeviceBattery>,
}

impl UPSPersistentWatcher {
    pub fn start(executor: UPSExecutorHandle, db: DatabaseConnection) -> JoinHandle<()> {
        let watcher = Self {
            executor,
            db,
            last_device_state: None,
            last_battery_state: None,
        };
        tokio::spawn(watcher.process())
    }

    /// Handle polling the device state at the expected interval
    /// and checking any changes
    pub async fn process(mut self) {
        let start = Instant::now() + POLL_INTERVAL;
        let mut interval = interval_at(start, POLL_INTERVAL);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        while self.executor.is_open() {
            self.process_device_state().await;
            self.process_device_battery().await;

            interval.tick().await;
        }
    }

    /// Requests the current device state saving it to the
    /// database
    pub async fn process_device_state(&mut self) {
        let device_state = match self.executor.request(QueryDeviceState).await {
            Ok(value) => value,
            Err(err) => {
                error!("Error while requesting UPS device state: {err:?}");
                return;
            }
        };

        let is_changed = self
            .last_device_state
            .as_ref()
            .map(|state| state.ne(&device_state))
            .unwrap_or_default();

        if is_changed {
            debug!("device state has changed")
        }

        let current_time = Utc::now();

        if let Err(err) =
            StateHistoryModel::create(&self.db, device_state.clone(), current_time).await
        {
            error!("failed to store current device state: {err}");
        }

        self.last_device_state = Some(device_state);
    }

    /// Requests the current device battery state saving it to the
    /// database
    pub async fn process_device_battery(&mut self) {
        let battery_state = match self.executor.request(QueryDeviceBattery).await {
            Ok(value) => value,
            Err(err) => {
                error!("Error while requesting UPS device battery: {err:?}");
                return;
            }
        };

        let is_changed = self
            .last_battery_state
            .as_ref()
            .map(|state| state.ne(&battery_state))
            .unwrap_or_default();

        if is_changed {
            debug!("device battery state has changed")
        }

        let current_time = Utc::now();

        if let Err(err) =
            BatteryHistoryModel::create(&self.db, battery_state.clone(), current_time).await
        {
            error!("failed to store current battery state: {err}");
        }

        self.last_battery_state = Some(battery_state);
    }
}
