//! Service that listens to a [UPSWatcherHandle] and stores any events
//! that occur in the database

use chrono::Utc;
use log::error;
use sea_orm::DatabaseConnection;

use crate::{database::entities::events::EventModel, services::watcher::UPSWatcherHandle};

pub struct UPSEventTracker {
    /// Database connection to store the data
    db: DatabaseConnection,
    /// Watcher handle to listen for events
    watcher_handle: UPSWatcherHandle,
}

impl UPSEventTracker {
    pub fn start(db: DatabaseConnection, watcher_handle: UPSWatcherHandle) {
        let tracker = Self { db, watcher_handle };
        tokio::spawn(tracker.process());
    }

    pub async fn process(mut self) {
        while let Some(event) = self.watcher_handle.next().await {
            let current_time = Utc::now();
            if let Err(err) = EventModel::create(&self.db, event, current_time).await {
                error!("failed to save event to database: {err}");
            }
        }
    }
}
