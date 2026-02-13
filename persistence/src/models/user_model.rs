use sqlx::types::chrono::{DateTime, Utc};

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub verified: bool,
    pub profile_picture: Option<String>,
    pub biography: Option<String>,
    pub created_at: DateTime<Utc>,
}