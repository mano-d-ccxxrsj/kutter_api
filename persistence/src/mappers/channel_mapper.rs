use domain::entities::channel_entity::Channel;

use crate::models::channel_model::ChannelModel;

pub struct ChannelMapper;

impl ChannelMapper {
    pub fn from_model(model: ChannelModel) -> Channel {
        Channel {
            id: model.id,
            community_id: model.community_id,
            name: model.name,
            topic: model.topic,
            hidden: model.hidden,
        }
    }
}