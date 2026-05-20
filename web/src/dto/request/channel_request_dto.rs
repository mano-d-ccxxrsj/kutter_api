use crate::dto::request::types::{DeserializeManual, JsonFieldReader};

pub struct CreateChannelRequestDto {
    pub community_id: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}

impl DeserializeManual for CreateChannelRequestDto {
    fn from_json(json: &str) -> Result<CreateChannelRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(CreateChannelRequestDto {
            community_id: reader.required_i32("community_id")?,
            name: reader.required_string("name")?,
            topic: reader.optional_string("topic")?,
            hidden: reader.required_bool("hidden")?,
        })
    }
}