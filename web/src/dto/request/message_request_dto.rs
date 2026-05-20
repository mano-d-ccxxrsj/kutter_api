use crate::dto::request::types::{DeserializeManual, JsonFieldReader};

pub struct SendChannelMessageRequestDto {
    pub community_id: i32,
    pub channel_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
}

pub struct ListChannelMessagesRequestDto {
    pub community_id: i32,
    pub channel_id: i32,
}

pub struct EditChannelMessageRequestDto {
    pub message_id: i32,
    pub message: String,
}

pub struct DeleteChannelMessageRequestDto {
    pub message_id: i32,
}

impl DeserializeManual for SendChannelMessageRequestDto {
    fn from_json(json: &str) -> Result<SendChannelMessageRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(SendChannelMessageRequestDto {
            community_id: reader.required_i32("community_id")?,
            channel_id: reader.required_i32("channel_id")?,
            message: reader.required_string("message")?,
            replied_message: reader.optional_i32("replied_message")?,
        })
    }
}

impl DeserializeManual for ListChannelMessagesRequestDto {
    fn from_json(json: &str) -> Result<ListChannelMessagesRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(ListChannelMessagesRequestDto {
            community_id: reader.required_i32("community_id")?,
            channel_id: reader.required_i32("channel_id")?,
        })
    }
}

impl DeserializeManual for EditChannelMessageRequestDto {
    fn from_json(json: &str) -> Result<EditChannelMessageRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(EditChannelMessageRequestDto {
            message_id: reader.required_i32("message_id")?,
            message: reader.required_string("message")?,
        })
    }
}

impl DeserializeManual for DeleteChannelMessageRequestDto {
    fn from_json(json: &str) -> Result<DeleteChannelMessageRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(DeleteChannelMessageRequestDto {
            message_id: reader.required_i32("message_id")?,
        })
    }
}