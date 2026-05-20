use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct TokenModel {
    pub id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
    pub revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
}