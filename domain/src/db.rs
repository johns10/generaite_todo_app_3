use sea_orm::DatabaseConnection;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sea_orm::DbErr> {
        let connection = sea_orm::Database::connect(database_url).await?;
        Ok(Self { connection })
    }
}
