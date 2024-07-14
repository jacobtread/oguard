use anyhow::Context;
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};

use crate::config::LoggingConfig;

/// The pattern to use when logging
const LOGGING_PATTERN: &str = "[{d} {h({l})} {M}] {m}{n}";

/// Log file name
pub const LOG_FILE_NAME: &str = "data/server.log";

/// Setup function for setting up the Log4rs logging configuring it
/// for all the different modules and and setting up file and stdout logging
pub fn setup(logging: &LoggingConfig) -> anyhow::Result<()> {
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

    const APPENDERS: [&str; 2] = ["stdout", "file"];

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", console))
        .appender(Appender::builder().build("file", file))
        .logger(
            Logger::builder()
                .appenders(APPENDERS)
                .additive(false)
                .build("oguard", logging_level),
        )
        .build(
            Root::builder()
                .appenders(APPENDERS)
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