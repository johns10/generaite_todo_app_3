use config::{Config as ConfigBuilder, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "dev".to_string());
        let config_path = format!("config/{}.toml", environment);

        let config = ConfigBuilder::builder()
            .add_source(File::with_name(&config_path))
            .build()?;

        config.try_deserialize()
    }
}
