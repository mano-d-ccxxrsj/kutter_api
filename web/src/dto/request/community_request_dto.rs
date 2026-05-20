use crate::dto::request::types::{DeserializeManual, JsonFieldReader};

pub struct CreateCommunityRequestDto {
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}

impl DeserializeManual for CreateCommunityRequestDto {
    fn from_json(json: &str) -> Result<CreateCommunityRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(CreateCommunityRequestDto {
            name: reader.required_string("name")?,
            about: reader.optional_string("about")?,
            nsfw: reader.required_bool("nsfw")?,
        })
    }
}