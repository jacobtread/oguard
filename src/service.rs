use anyhow::Context;
use log::{debug, error};
use std::env;
use std::ffi::OsString;
use std::process::Command;
use std::time::Duration;
use tokio::sync::mpsc;
use windows_service::service::{
    ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler;
use windows_service::service_control_handler::ServiceControlHandlerResult;

use crate::config::Config;
use crate::{config, logging, server::run_server};

pub const SERVICE_NAME: &str = "oguard";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn service_main(_arguments: Vec<OsString>) {
    // Get the path to the executable
    let exe_path = env::current_exe().expect("Failed to get current executable path");

    // Navigate to the directory containing the executable
    let exe_dir = exe_path.parent().expect("Failed to get parent directory");

    // Set the working directory to the executable's directory
    env::set_current_dir(exe_dir).expect("Failed to set current directory");

    // Load the configuration
    let config = config::load_default();

    // Setup logging
    logging::setup(&config.logging).expect("failed to setup logging");

    if let Err(err) = run_service(config) {
        error!("error running service: {err}")
    }
}

pub fn restart_service() -> anyhow::Result<()> {
    stop_service()?;
    start_service()?;
    Ok(())
}

pub fn start_service() -> anyhow::Result<()> {
    debug!("starting service");

    Command::new("sc")
        .args(["start", SERVICE_NAME])
        .output()
        .context("failed to start service")?;
    Ok(())
}

pub fn stop_service() -> anyhow::Result<()> {
    debug!("stopping service");

    Command::new("sc")
        .args(["stop", SERVICE_NAME])
        .output()
        .context("failed to stop service")?;

    Ok(())
}

fn run_service(config: Config) -> anyhow::Result<()> {
    // Create a channel to be able to poll a stop event from the service worker loop.
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    // Define system service event handler that will be receiving service events.
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            // Notifies a service to report its current status information to the service
            // control manager. Always return NoError even if not implemented.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

            // Handle stop
            ServiceControl::Stop => {
                _ = shutdown_tx.try_send(());
                ServiceControlHandlerResult::NoError
            }

            // treat the UserEvent as a stop request
            ServiceControl::UserEvent(code) => {
                if code.to_raw() == 130 {
                    _ = shutdown_tx.try_send(());
                }
                ServiceControlHandlerResult::NoError
            }

            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler.
    // The returned status handle should be used to report service status changes to the system.
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    // Tell the system that service is running
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    runtime.block_on(run_server(config, shutdown_rx))?;

    // Tell the system that service has stopped.
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}
