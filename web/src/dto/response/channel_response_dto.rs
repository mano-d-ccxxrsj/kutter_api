use domain::entities::channel_entity::Channel;

use crate::dto::response::types::{JsonObjectBuilder, SerializeManual};

pub struct ChannelResponseDto {
    pub id: i32,
    pub community_id: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}

impl From<Channel> for ChannelResponseDto {
    fn from(channel: Channel) -> Self {
        Self {
            id: channel.id,
            community_id: channel.community_id,
            name: channel.name,
            topic: channel.topic,
            hidden: channel.hidden,
        }
    }
}

impl SerializeManual for ChannelResponseDto {
    fn serialize_json(&self) -> String {
        let json: String = JsonObjectBuilder::new()
            .with_i32("id", self.id)
            .with_i32("community_id", self.community_id)
            .with_string("name", &self.name)
            .with_optional_string("topic", &self.topic)
            .with_bool("hidden", self.hidden)
            .finish();

        json
    }
}