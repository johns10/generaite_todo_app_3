use crate::user::model::{User, UserForm};
use ::entity::user;
use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("User not found")]
    NotFound,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, form: UserForm) -> Result<User, UserRepositoryError>;
    async fn get(&self, id: Uuid) -> Result<User, UserRepositoryError>;
    async fn update(&self, id: Uuid, form: UserForm) -> Result<User, UserRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<u64, UserRepositoryError>;
    async fn list(&self) -> Result<Vec<User>, UserRepositoryError>;
}

pub struct UserRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, form: UserForm) -> Result<User, UserRepositoryError> {
        let user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(form.username),
            email: Set(form.email),
            password_hash: Set("".to_string()), // You should implement password hashing
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        };

        let user = user.insert(&*self.conn).await?;
        Ok(User::from(user))
    }

    async fn get(&self, id: Uuid) -> Result<User, UserRepositoryError> {
        let user = user::Entity::find_by_id(id)
            .one(&*self.conn)
            .await?
            .ok_or(UserRepositoryError::NotFound)?;
        Ok(User::from(user))
    }

    async fn update(&self, id: Uuid, form: UserForm) -> Result<User, UserRepositoryError> {
        let mut user: user::ActiveModel = user::Entity::find_by_id(id)
            .one(&*self.conn)
            .await?
            .ok_or(UserRepositoryError::NotFound)?
            .into();

        user.username = Set(form.username);
        user.email = Set(form.email);
        user.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated_user = user.update(&*self.conn).await?;
        Ok(User::from(updated_user))
    }

    async fn delete(&self, id: Uuid) -> Result<u64, UserRepositoryError> {
        let result = user::Entity::delete_by_id(id)
            .exec(self.conn.as_ref())
            .await
            .map(|res| res.rows_affected)
            .map_err(UserRepositoryError::from);
    }

    async fn list(&self) -> Result<Vec<User>, UserRepositoryError> {
        let users = user::Entity::find().all(&*self.conn).await?;
        Ok(users.into_iter().map(User::from).collect())
    }
}
