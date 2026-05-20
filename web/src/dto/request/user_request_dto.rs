use domain::types::auth_types::{LoginUserCommand, RegisterUserCommand};

use crate::dto::request::types::{DeserializeManual, JsonFieldReader};

pub struct RegisterUserRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginUserRequestDto {
    pub email: String,
    pub password: String,
}

impl DeserializeManual for RegisterUserRequestDto {
    fn from_json(json: &str) -> Result<RegisterUserRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(RegisterUserRequestDto {
            username: reader.required_string("username")?,
            email: reader.required_string("email")?,
            password: reader.required_string("password")?,
        })
    }
}

impl DeserializeManual for LoginUserRequestDto {
    fn from_json(json: &str) -> Result<LoginUserRequestDto, String> {
        let reader: JsonFieldReader = JsonFieldReader::new(json)?;

        Ok(LoginUserRequestDto {
            email: reader.required_string("email")?,
            password: reader.required_string("password")?,
        })
    }
}

impl From<RegisterUserRequestDto> for RegisterUserCommand {
    fn from(dto: RegisterUserRequestDto) -> Self {
        Self {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }
}

impl From<LoginUserRequestDto> for LoginUserCommand {
    fn from(dto: LoginUserRequestDto) -> Self {
        Self {
            email: dto.email,
            password: dto.password,
        }
    }
}