use std::time::Duration;

pub struct LocalShutdownBehavior {
    /// On battery behavior
    pub battery: UPSBatteryBehavior,
    /// Behavior for low battery
    pub low_battery: UPSBatteryLowBehavior,
    /// Type of shutdown
    pub mode: ShutdownMode,
    /// Delay before the shutdown is executed
    pub shutdown_delay: Duration,
    /// Program to execute before shutting down
    pub shutdown_execute: Option<ExecuteConfig>,
    /// Program to execute on shutdown cancelled
    pub cancel_shutdown_execute: Option<ExecuteConfig>,
    /// Warning dialog delays and settings
    pub warning: WarningDialogConfig,
}

pub enum ShutdownMode {
    Shutdown,
    Sleep,
}

/// UPS Behavior when on battery (When battery is not low)
pub enum UPSBatteryBehavior {
    /// Don't shut down the system but execute the provided command and the provided duration
    ExecuteAfter(Duration),

    /// Shutdown the UPS on battery
    Shutdown {
        /// Shutdown after a specific duration
        after: Option<Duration>,
        /// Shutdown when below a specified battery capacity
        below_capacity: Option<u8>,
        /// Also shutdown the UPS after shutting down the local system
        shutdown_ups: bool,
    },
}

pub struct UPSBatteryLowBehavior {
    /// Whether to shutdown the system immediately on low battery
    pub shutdown_immediately: bool,

    /// Behavior of the UPS after shutdown
    pub ups: UPSShutdown,
}

/// Behavior of the UPS after shutting down the local system
pub enum UPSShutdown {
    /// Decide base on the model
    BasedOnModel,
    /// Shut down the UPS
    ShutdownImmediately,
    /// Leave the UPS on
    StayOn,
}

pub struct ExecuteConfig {
    /// The path to the program executable
    pub program: String,
    /// Maximum time the program is allowed to run for
    pub max_execution_time: Option<Duration>,
}

pub struct WarningDialogConfig {
    /// Time before shutdown that a warning dialog should be shown
    pub before_shutdown_delay: Duration,
    /// Interval warnings should repeat at until the shutdown
    pub repeat_interval: Duration,
}

pub struct RemoteShutdown {
    // TODO: Remote shutdown options
}
