use crate::user::model::{User, UserForm};
use async_trait::async_trait;
use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
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

pub type Result<T> = std::result::Result<T, UserRepositoryError>;

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(&self, form: UserForm) -> Result<User>;
    async fn get(&self, id: Uuid) -> Result<User>;
    async fn update(&self, id: Uuid, form: UserForm) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self) -> Result<Vec<User>>;
}

pub struct UserRepository {
    conn: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create(&self, form: UserForm) -> Result<User> {
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

    async fn get(&self, id: Uuid) -> Result<User> {
        let user = user::Entity::find_by_id(id)
            .one(&*self.conn)
            .await?
            .ok_or(UserRepositoryError::NotFound)?;
        Ok(User::from(user))
    }

    async fn update(&self, id: Uuid, form: UserForm) -> Result<User> {
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

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = user::Entity::delete_by_id(id).exec(&*self.conn).await?;
        if result.rows_affected == 0 {
            return Err(UserRepositoryError::NotFound);
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>> {
        let users = user::Entity::find().all(&*self.conn).await?;
        Ok(users.into_iter().map(User::from).collect())
    }
}
