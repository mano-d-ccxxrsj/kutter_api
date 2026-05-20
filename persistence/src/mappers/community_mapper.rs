use domain::entities::community_entity::Community;

use crate::models::community_model::CommunityModel;

pub struct CommunityMapper;

impl CommunityMapper {
    pub fn from_model(model: CommunityModel) -> Community {
        Community {
            id: model.id,
            name: model.name,
            about: model.about,
            nsfw: model.nsfw,
        }
    }
}