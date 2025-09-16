use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
    sync::Arc,
};

use anyhow::Context;
use log::{LevelFilter, error};
use serde::Deserialize;
use std::fs::read_to_string;

/// Linux release builds load config from /etc/oguard
#[cfg(all(target_os = "linux", not(debug_assertions)))]
const CONFIG_PATH: &str = "/etc/oguard/config.toml";

/// Macos release builds load config from /Library/Application Support/oguard
#[cfg(all(target_os = "linux", not(debug_assertions)))]
const CONFIG_PATH: &str = "/Library/Application Support/oguard/config.toml";

/// Windows and debug builds load config from the working directory
#[cfg(any(windows, debug_assertions))]
const CONFIG_PATH: &str = "config.toml";

pub type SharedConfig = Arc<Config>;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Locale to use when creating error messages and prompts
    pub locale: String,
    /// HTTP configuration
    pub http: HttpConfig,
    /// Login configuration and credentials
    pub login: LoginConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            http: Default::default(),
            login: Default::default(),
            logging: Default::default(),
        }
    }
}

/// Configurations for logging
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    /// Logging level filter
    pub level: LevelFilter,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LevelFilter::Info,
        }
    }
}

/// Configurations for the HTTP server
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    /// Host to bind the server on
    pub host: IpAddr,
    /// Port to bind the server on
    pub port: u16,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            host: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 3000,
        }
    }
}

/// Login configuration, by default there is no credentials
/// and the server cannot be logged in until the user sets
/// credentials in the file
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct LoginConfig {
    /// Password, if not set login will not be allowed
    pub password: Option<String>,
}

/// Loads the user configuration from their expected configuration
/// path see [CONFIG_PATH]
pub fn load_user() -> Config {
    let path = Path::new(CONFIG_PATH);
    match from_file(path) {
        Ok(value) => value,
        Err(err) => {
            error!("failed to load config file, using defaults: {err}");
            Config::default()
        }
    }
}

/// Loads a configuration from a file
pub fn from_file(path: &Path) -> anyhow::Result<Config> {
    let value = read_to_string(path).context("read config file")?;
    from_str(&value)
}

/// Loads a configuration from a string
pub fn from_str(value: &str) -> anyhow::Result<Config> {
    toml::from_str(value).context("failed to parse config")
}
