use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use rust_i18n::i18n;
use service::{restart_service, start_service, stop_service};
use tokio::sync::mpsc;

pub mod action;
pub mod config;
pub mod database;
pub mod http;
pub mod logging;
pub mod persistent_watcher;
pub mod server;
pub mod service;
pub mod ups;
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
    Service(ServiceArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct ServiceArgs {
    #[command(subcommand)]
    command: ServiceCommands,
}

#[derive(Debug, Subcommand)]
enum ServiceCommands {
    Start,
    Stop,
    Restart,
}

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        // Load the configuration
        let config = config::load_default();

        // Setup logging
        logging::setup(&config.logging).expect("failed to setup logging");

        return match command {
            Commands::Service(service) => match service.command {
                ServiceCommands::Start => start_service(),
                ServiceCommands::Stop => stop_service(),
                ServiceCommands::Restart => restart_service(),
            },
        };
    }

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
