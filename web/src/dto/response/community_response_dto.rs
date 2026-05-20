use domain::entities::community_entity::Community;

use crate::dto::response::types::{JsonObjectBuilder, SerializeManual};

pub struct CommunityResponseDto {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}

impl From<Community> for CommunityResponseDto {
    fn from(community: Community) -> Self {
        Self {
            id: community.id,
            name: community.name,
            about: community.about,
            nsfw: community.nsfw,
        }
    }
}

impl SerializeManual for CommunityResponseDto {
    fn serialize_json(&self) -> String {
        let json: String = JsonObjectBuilder::new()
            .with_i32("id", self.id)
            .with_string("name", &self.name)
            .with_optional_string("about", &self.about)
            .with_bool("nsfw", self.nsfw)
            .finish();

        json
    }
}