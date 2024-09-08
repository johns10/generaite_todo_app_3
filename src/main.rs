use std::sync::Arc;
use domain::{db, repository::Repository};

mod config;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new().expect("Failed to load config");

    let conn = Arc::new(db::initialize(&config.database.url).await?);
    let repository = Repository::new(conn);

    // You can now use the `repository` for database operations

    Ok(())
}
