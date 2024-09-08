use crate::user::model::{User, UserForm};
use anyhow::Result;
use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRepository {
    conn: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn create(&self, form: UserForm) -> Result<User> {
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

    pub async fn get(&self, id: Uuid) -> Result<Option<User>> {
        let user = user::Entity::find_by_id(id).one(&*self.conn).await?;
        Ok(user.map(User::from))
    }

    pub async fn update(&self, id: Uuid, form: UserForm) -> Result<User> {
        let mut user: user::ActiveModel = user::Entity::find_by_id(id)
            .one(&*self.conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?
            .into();

        user.username = Set(form.username);
        user.email = Set(form.email);
        user.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated_user = user.update(&*self.conn).await?;
        Ok(User::from(updated_user))
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        user::Entity::delete_by_id(id).exec(&*self.conn).await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<User>> {
        let users = user::Entity::find().all(&*self.conn).await?;
        Ok(users.into_iter().map(User::from).collect())
    }
}
