use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use rust_i18n::i18n;
use tokio::sync::mpsc;

pub mod action;
pub mod config;
pub mod database;
pub mod http;
pub mod logging;
pub mod server;
#[cfg(target_os = "windows")]
pub mod service;
pub mod services;
pub mod ups;
pub mod utils;
pub mod watcher;

// Initialize localization
i18n!("locales", fallback = "en");

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interact with the oguard system service
    #[cfg(target_os = "windows")]
    Service(ServiceArgs),
}

#[cfg(target_os = "windows")]
#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct ServiceArgs {
    #[command(subcommand)]
    command: ServiceCommands,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Subcommand)]
enum ServiceCommands {
    /// Create the service (Will fail if the service is already created)
    Create,
    /// Start the service
    Start,
    /// Stop the service
    Stop,
    /// Restart the service
    Restart,
    /// Delete the service
    Delete,
}

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        // Load the configuration
        let config = config::load_default();

        // Setup logging
        logging::setup(&config.logging).expect("failed to setup logging");

        #[allow(unreachable_code)]
        return match command {
            #[cfg(target_os = "windows")]
            Commands::Service(service) => match service.command {
                ServiceCommands::Create => service::create_service(),
                ServiceCommands::Start => service::start_service(),
                ServiceCommands::Stop => service::stop_service(),
                ServiceCommands::Restart => service::restart_service(),
                ServiceCommands::Delete => service::delete_service(),
            },
        };
    }

    #[cfg(target_os = "windows")]
    {
        // Debug builds run the server directly
        #[cfg(debug_assertions)]
        {
            server_main()?;
        }

        // Production builds start the service logic
        #[cfg(not(debug_assertions))]
        {
            windows_service::service_dispatcher::start(service::SERVICE_NAME, ffi_service_main)
                .context("failed to start service")?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        server_main()?;
    }

    Ok(())
}

#[cfg(not(debug_assertions))]
#[doc = r" Static callback used by the system to bootstrap the service."]
#[doc = r" Do not call it directly."]
extern "system" fn ffi_service_main(num_service_arguments: u32, service_arguments: *mut *mut u16) {
    let arguments = unsafe {
        windows_service::service_dispatcher::parse_service_arguments(
            num_service_arguments,
            service_arguments,
        )
    };
    service::service_main(arguments);
}

#[cfg(debug_assertions)]
fn server_main() -> anyhow::Result<()> {
    // Load the configuration
    let config = config::load_default();

    // Setup logging
    logging::setup(&config.logging).context("failed to setup logging")?;

    // Create a channel to safely shutdown the server when requested
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    // Listen to the shutdown signal
    runtime.spawn(async move {
        _ = tokio::signal::ctrl_c().await;
        _ = shutdown_tx.send(()).await;
    });

    runtime.block_on(server::run_server(config, shutdown_rx))
}
