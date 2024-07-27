use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
    sync::Arc,
};

use anyhow::Context;
use log::{error, LevelFilter};
use serde::Deserialize;
use std::fs::read_to_string;

/// On windows the configuration is loaded from the working directory
#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "config.toml";

/// Linux release builds load config from /etc/oguard
#[cfg(all(target_os = "linux", not(debug_assertions)))]
const CONFIG_PATH: &str = "/etc/oguard/config.toml";

/// Linux debug builds load config from the working directory
#[cfg(all(target_os = "linux", debug_assertions))]
const CONFIG_PATH: &str = "config.toml";

pub type SharedConfig = Arc<Config>;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub locale: String,
    pub http: HttpConfig,
    pub login: LoginConfig,
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

/// Configurations for the HTTP server
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
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

pub fn load_default() -> Config {
    let path = Path::new(CONFIG_PATH);
    match from_file(path) {
        Ok(value) => value,
        Err(err) => {
            error!("failed to load config file, using defaults: {err}");
            Config::default()
        }
    }
}

pub fn from_file(path: &Path) -> anyhow::Result<Config> {
    let value = read_to_string(path).context("read config file")?;
    from_str(&value)
}

pub fn from_str(value: &str) -> anyhow::Result<Config> {
    toml::from_str(value).context("failed to parse config")
}
