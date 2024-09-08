use chrono::{DateTime, NaiveDateTime, Utc};
use entity::user;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
}

impl From<user::Model> for User {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            created_at: naive_to_utc(model.created_at),
            updated_at: naive_to_utc(model.updated_at),
        }
    }
}

impl From<user::ActiveModel> for User {
    fn from(active_model: user::ActiveModel) -> Self {
        Self {
            id: active_model.id.unwrap(),
            username: active_model.username.unwrap(),
            email: active_model.email.unwrap(),
            created_at: naive_to_utc(active_model.created_at.unwrap()),
            updated_at: naive_to_utc(active_model.updated_at.unwrap()),
        }
    }
}

fn naive_to_utc(naive: NaiveDateTime) -> DateTime<Utc> {
    DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
}
