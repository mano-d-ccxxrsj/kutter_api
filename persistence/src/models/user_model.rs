use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub public_key: Vec<u8>,
    pub verified: bool,
    pub profile_picture: Option<String>,
    pub biography: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}