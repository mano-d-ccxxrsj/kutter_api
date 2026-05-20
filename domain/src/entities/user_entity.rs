use chrono::{DateTime, Utc};

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub public_key: [u8; 32],
    pub verified: bool,
    pub profile_picture: Option<String>,
    pub biography: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub public_key: [u8; 32],
    pub verified: bool,
    pub profile_picture: Option<String>,
    pub biography: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}