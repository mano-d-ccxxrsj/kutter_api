use domain::entities::message_entity::ChannelMessage;

use crate::models::message_model::MessageModel;

pub struct MessageMapper;

impl MessageMapper {
    pub fn from_model(model: MessageModel) -> ChannelMessage {
        ChannelMessage {
            id: model.id,
            channel_id: model.channel_id,
            user_id: model.user_id,
            message: model.message,
            replied_message: model.replied_message,
            timestamp: model.timestamp,
            edited: model.edited,
        }
    }
}