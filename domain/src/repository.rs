use crate::user::repository::UserRepository;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct Repository {
    pub user: UserRepository,
}

impl Repository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            user: UserRepository::new(Arc::clone(&conn)),
        }
    }
}
