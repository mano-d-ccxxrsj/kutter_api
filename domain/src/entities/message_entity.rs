use chrono::{DateTime, Utc};

pub struct ChannelMessage {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
    pub timestamp: DateTime<Utc>,
    pub edited: bool,
}

pub struct NewChannelMessage {
    pub channel_id: i32,
    pub user_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
    pub timestamp: DateTime<Utc>,
    pub edited: bool,
}