use serde::Deserialize;
use std::fs;
use std::env;
use toml;

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
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "dev".to_string());
        let config_path = format!("config/{}.toml", environment);
        let config_str = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
