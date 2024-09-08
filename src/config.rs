use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("Invalid SERVER_PORT"),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                pool_size: env::var("DATABASE_POOL_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .expect("Invalid DATABASE_POOL_SIZE"),
            },
        })
    }
}
