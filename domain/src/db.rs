use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn initialize(db_url: &str) -> anyhow::Result<DatabaseConnection> {
    let conn = Database::connect(db_url).await?;
    Migrator::up(&conn, None).await?;
    Ok(conn)
}
