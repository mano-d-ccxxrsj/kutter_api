use domain::entities::message_entity::ChannelMessage;

use crate::dto::response::types::{JsonObjectBuilder, SerializeManual};

pub struct ChannelMessageResponseDto {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
    pub timestamp: String,
    pub edited: bool,
}

impl From<ChannelMessage> for ChannelMessageResponseDto {
    fn from(message: ChannelMessage) -> Self {
        Self {
            id: message.id,
            channel_id: message.channel_id,
            user_id: message.user_id,
            message: message.message,
            replied_message: message.replied_message,
            timestamp: message.timestamp.to_rfc3339(),
            edited: message.edited,
        }
    }
}

impl SerializeManual for ChannelMessageResponseDto {
    fn serialize_json(&self) -> String {
        let json: String = JsonObjectBuilder::new()
            .with_i32("id", self.id)
            .with_i32("channel_id", self.channel_id)
            .with_i32("user_id", self.user_id)
            .with_string("message", &self.message)
            .with_optional_i32("replied_message", self.replied_message)
            .with_string("timestamp", &self.timestamp)
            .with_bool("edited", self.edited)
            .finish();

        json
    }
}