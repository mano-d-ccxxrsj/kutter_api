use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct UserFlagModel {
    pub id: i32,
    pub user_id: i32,
    pub field: String,
    pub action: String,
    pub target: String,
    pub attempted_text: String,
    pub matched_words: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}