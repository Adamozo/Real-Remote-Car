use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlParseError(#[from] serde_yaml::Error),

    #[error("Configuration validation error: {0}")]
    ValidationError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub client_id: String,
    #[serde(default = "default_keep_alive")]
    pub keep_alive_secs: u64,
    #[serde(default = "default_reconnect_interval")]
    pub reconnect_interval_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LogConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_file")]
    pub file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub mqtt: MqttConfig,
    #[serde(default)]
    pub logging: LogConfig,
    #[serde(default = "default_update_interval")]
    pub update_interval_ms: u64,
}

fn default_keep_alive() -> u64 {
    60
}
fn default_reconnect_interval() -> u64 {
    5
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_file() -> Option<String> {
    None
}
fn default_update_interval() -> u64 {
    1000
}

impl AppConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let config_str = fs::read_to_string(path)?;

        let config: AppConfig = serde_yaml::from_str(&config_str)?;

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.mqtt.host.is_empty() {
            return Err(ConfigError::ValidationError(
                "MQTT host cannot be empty".to_string(),
            ));
        }

        if self.mqtt.port == 0 {
            return Err(ConfigError::ValidationError(
                "MQTT port cannot be 0".to_string(),
            ));
        }

        if self.mqtt.keep_alive_secs == 0 {
            return Err(ConfigError::ValidationError(
                "MQTT keep alive cannot be 0".to_string(),
            ));
        }

        if self.update_interval_ms == 0 {
            return Err(ConfigError::ValidationError(
                "Update interval cannot be 0".to_string(),
            ));
        }

        let valid_log_levels = ["debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&self.logging.level.to_lowercase().as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "Invalid log level: {}. Must be one of: {:?}",
                self.logging.level, valid_log_levels
            )));
        }

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let yaml = serde_yaml::to_string(self)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    pub fn default_config() -> Self {
        AppConfig {
            mqtt: MqttConfig {
                host: "localhost".to_string(),
                port: 1883,
                username: "default_user".to_string(),
                password: "default_password".to_string(),
                client_id: "rust-client".to_string(),
                keep_alive_secs: default_keep_alive(),
                reconnect_interval_secs: default_reconnect_interval(),
            },
            logging: LogConfig {
                level: default_log_level(),
                file: default_log_file(),
            },
            update_interval_ms: default_update_interval(),
        }
    }
}
