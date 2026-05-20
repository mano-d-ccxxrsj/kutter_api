use actix_web::{HttpResponse, web};
use std::string::FromUtf8Error;

pub struct RequestBody;

impl RequestBody {
    pub fn string(body: web::Bytes) -> Result<String, HttpResponse> {
        let text: String = String::from_utf8(body.to_vec())
            .map_err(|error: FromUtf8Error| HttpResponse::BadRequest().body(error.to_string()))?;

        Ok(text)
    }
}