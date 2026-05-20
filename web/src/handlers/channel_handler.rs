use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, web};
use domain::entities::channel_entity::Channel;
use domain::errors::auth_error::AuthError;
use domain::errors::channel_error::ChannelError;
use domain::ports::services::service_set_port::ServiceSetPort;
use domain::types::auth_types::AuthenticatedSession;
use domain::types::channel_types::CreateChannelCommand;

use crate::dto::request::channel_request_dto::CreateChannelRequestDto;
use crate::dto::request::types::DeserializeManual;
use crate::dto::response::channel_response_dto::ChannelResponseDto;
use crate::dto::response::types::SerializeManual;
use crate::handlers::request_body::RequestBody;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/channel")
            .service(web::resource("/create").route(web::post().to(create))),
    );
}

pub async fn create(
    request: HttpRequest,
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let session_token: String = match request.cookie("session") {
        Some(cookie) => cookie.value().to_string(),
        None => return HttpResponse::Unauthorized().body("Session cookie is required"),
    };

    let session: AuthenticatedSession = match services
        .auth_service()
        .authenticate_session(session_token)
        .await
    {
        Ok(authenticated) => authenticated,
        Err(error) => return auth_error_response(error),
    };

    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: CreateChannelRequestDto = match CreateChannelRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: CreateChannelCommand = CreateChannelCommand {
        user_id: session.user_id,
        community_id: dto.community_id,
        name: dto.name,
        topic: dto.topic,
        hidden: dto.hidden,
    };

    let response: HttpResponse = match services.channel_service().create(command).await {
        Ok(channel) => {
            let created: Channel = channel;
            let dto: ChannelResponseDto = created.into();

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(dto.serialize_json())
        }
        Err(error) => channel_error_response(error),
    };

    response
}

fn auth_error_response(error: AuthError) -> HttpResponse {
    let response: HttpResponse = match error {
        AuthError::EmailAlreadyRegistered | AuthError::UsernameAlreadyRegistered => {
            HttpResponse::BadRequest().body(error.to_string())
        }
        AuthError::InvalidCredentials | AuthError::Security(_) => {
            HttpResponse::Unauthorized().body(error.to_string())
        }
        AuthError::Repository(_) => HttpResponse::InternalServerError().body(error.to_string()),
    };

    response
}

fn channel_error_response(error: ChannelError) -> HttpResponse {
    let response: HttpResponse = match error {
        ChannelError::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
        ChannelError::Repository(_) => HttpResponse::InternalServerError().body(error.to_string()),
    };

    response
}