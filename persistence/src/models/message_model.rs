use sqlx::types::chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct MessageModel {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
    pub timestamp: DateTime<Utc>,
    pub edited: bool,
}