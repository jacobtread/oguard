use anyhow::Context;
use log::LevelFilter;
use log4rs::{
    Config,
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    init_config,
};

use crate::config::LoggingConfig;

/// The pattern to use when logging
const LOGGING_PATTERN: &str = "[{d} {h({l})} {M}] {m}{n}";

/// Linux release builds save the log file to /usr/local/share/oguard/server.log
#[cfg(all(target_os = "linux", not(debug_assertions)))]
const LOG_FILE_NAME: &str = "/usr/local/share/oguard/server.log";

/// Macos release builds save the log file to /Library/Logs/oguard/server.log
#[cfg(all(target_os = "macos", not(debug_assertions)))]
const LOG_FILE_NAME: &str = "/Library/Logs/oguard/server.log";

/// Windows and debug builds the log file is saved to the working directory
#[cfg(any(windows, debug_assertions))]
const LOG_FILE_NAME: &str = "data/server.log";

/// Setup function for setting up the Log4rs logging configuring it
/// for all the different modules and and setting up file and stdout logging
pub fn setup(logging: &LoggingConfig, persist: bool) -> anyhow::Result<()> {
    let logging_level = logging.level;

    if logging_level == LevelFilter::Off {
        // Don't initialize logger at all if logging is disabled
        return Ok(());
    }

    // Create logging appenders
    let pattern = Box::new(PatternEncoder::new(LOGGING_PATTERN));
    let console = Box::new(ConsoleAppender::builder().encoder(pattern.clone()).build());
    let file = Box::new(
        FileAppender::builder()
            .encoder(pattern)
            .build(LOG_FILE_NAME)
            .context("failed to create logging file appender")?,
    );

    let mut config = Config::builder().appender(Appender::builder().build("stdout", console));

    let mut appenders = vec!["stdout".to_string()];

    // Only add file appender if persisting logs
    if persist {
        config = config.appender(Appender::builder().build("file", file));
        appenders.push("file".to_string());
    }

    let config = config
        .logger(
            Logger::builder()
                .appenders(&appenders)
                .additive(false)
                .build("oguard", logging_level),
        )
        .build(
            Root::builder()
                .appenders(&appenders)
                .build(LevelFilter::Warn),
        )
        .context("failed to create logging config")?;

    init_config(config).context("failed to initialize logger")?;

    // Include panics in logging
    log_panics::init();

    Ok(())
}

/// Sets up basic console logging for tests
#[cfg(test)]
pub fn setup_test_logging() {
    // Create logging appenders
    let pattern = Box::new(PatternEncoder::new(LOGGING_PATTERN));
    let console = Box::new(ConsoleAppender::builder().encoder(pattern.clone()).build());

    const APPENDERS: [&str; 1] = ["stdout"];

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", console))
        .logger(
            Logger::builder()
                .appenders(APPENDERS)
                .additive(false)
                .build("oguard", LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appenders(APPENDERS)
                .build(LevelFilter::Warn),
        )
        .expect("Failed to create logging config");

    init_config(config).expect("Unable to initialize logger");

    // Include panics in logging
    log_panics::init();
}
