use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct Repository {}

impl Repository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {}
    }
}
