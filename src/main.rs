use std::sync::Arc;
use domain::{db, repository::Repository};
use web::server;

mod config;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new().expect("Failed to load config");

    let conn = Arc::new(db::initialize(&config.database.url).await?);
    let repository = Repository::new(conn);

    let app = server::create_app(repository)?;
    
    println!("Starting server at {}:{}", config.server.host, config.server.port);
    server::start(app, &config.server.host, config.server.port).await?;

    Ok(())
}
