use crate::user::repository::{UserRepository, UserRepositoryImpl};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct Repository {
    pub user: Arc<dyn UserRepository + Send + Sync>,
}

impl Repository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            user: Arc::new(UserRepositoryImpl::new(conn)),
        }
    }
}
