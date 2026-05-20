use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, web};
use domain::entities::message_entity::ChannelMessage;
use domain::errors::auth_error::AuthError;
use domain::errors::message_error::MessageError;
use domain::ports::services::service_set_port::ServiceSetPort;
use domain::types::auth_types::AuthenticatedSession;
use domain::types::message_types::{
    DeleteChannelMessageCommand, EditChannelMessageCommand, ListChannelMessagesCommand,
    SendChannelMessageCommand,
};

use crate::dto::request::message_request_dto::{
    DeleteChannelMessageRequestDto, EditChannelMessageRequestDto, ListChannelMessagesRequestDto,
    SendChannelMessageRequestDto,
};
use crate::dto::request::types::DeserializeManual;
use crate::dto::response::message_response_dto::ChannelMessageResponseDto;
use crate::dto::response::types::SerializeManual;
use crate::handlers::request_body::RequestBody;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/channel/message")
            .service(web::resource("/send").route(web::post().to(send)))
            .service(web::resource("/list").route(web::post().to(list)))
            .service(web::resource("/edit").route(web::post().to(edit)))
            .service(web::resource("/delete").route(web::post().to(delete))),
    );
}

pub async fn send(
    request: HttpRequest,
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let session: AuthenticatedSession = match authenticate_request(request, services.as_ref().as_ref()).await {
        Ok(authenticated) => authenticated,
        Err(response) => return response,
    };

    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: SendChannelMessageRequestDto = match SendChannelMessageRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: SendChannelMessageCommand = SendChannelMessageCommand {
        user_id: session.user_id,
        community_id: dto.community_id,
        channel_id: dto.channel_id,
        message: dto.message,
        replied_message: dto.replied_message,
    };

    let response: HttpResponse = match services.message_service().send(command).await {
        Ok(message) => {
            let sent: ChannelMessage = message;
            let dto: ChannelMessageResponseDto = sent.into();

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(dto.serialize_json())
        }
        Err(error) => message_error_response(error),
    };

    response
}

pub async fn list(
    request: HttpRequest,
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let session: AuthenticatedSession = match authenticate_request(request, services.as_ref().as_ref()).await {
        Ok(authenticated) => authenticated,
        Err(response) => return response,
    };

    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: ListChannelMessagesRequestDto = match ListChannelMessagesRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: ListChannelMessagesCommand = ListChannelMessagesCommand {
        user_id: session.user_id,
        community_id: dto.community_id,
        channel_id: dto.channel_id,
    };

    let response: HttpResponse = match services.message_service().list(command).await {
        Ok(messages) => {
            let response_dto: Vec<ChannelMessageResponseDto> = messages
                .into_iter()
                .map(ChannelMessageResponseDto::from)
                .collect();

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response_dto.serialize_json())
        }
        Err(error) => message_error_response(error),
    };

    response
}

pub async fn edit(
    request: HttpRequest,
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let session: AuthenticatedSession = match authenticate_request(request, services.as_ref().as_ref()).await {
        Ok(authenticated) => authenticated,
        Err(response) => return response,
    };

    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: EditChannelMessageRequestDto = match EditChannelMessageRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: EditChannelMessageCommand = EditChannelMessageCommand {
        user_id: session.user_id,
        message_id: dto.message_id,
        message: dto.message,
    };

    let response: HttpResponse = match services.message_service().edit(command).await {
        Ok(message) => {
            let edited: ChannelMessage = message;
            let dto: ChannelMessageResponseDto = edited.into();

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(dto.serialize_json())
        }
        Err(error) => message_error_response(error),
    };

    response
}

pub async fn delete(
    request: HttpRequest,
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let session: AuthenticatedSession = match authenticate_request(request, services.as_ref().as_ref()).await {
        Ok(authenticated) => authenticated,
        Err(response) => return response,
    };

    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: DeleteChannelMessageRequestDto = match DeleteChannelMessageRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: DeleteChannelMessageCommand = DeleteChannelMessageCommand {
        user_id: session.user_id,
        message_id: dto.message_id,
    };

    let response: HttpResponse = match services.message_service().delete(command).await {
        Ok(()) => HttpResponse::Ok().body("message deleted successfully"),
        Err(error) => message_error_response(error),
    };

    response
}

async fn authenticate_request(
    request: HttpRequest,
    services: &dyn ServiceSetPort,
) -> Result<AuthenticatedSession, HttpResponse> {
    let session_token: String = match request.cookie("session") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(HttpResponse::Unauthorized().body("Session cookie is required")),
    };

    let session: AuthenticatedSession = match services
        .auth_service()
        .authenticate_session(session_token)
        .await
    {
        Ok(authenticated) => authenticated,
        Err(error) => return Err(auth_error_response(error)),
    };

    Ok(session)
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

fn message_error_response(error: MessageError) -> HttpResponse {
    let response: HttpResponse = match error {
        MessageError::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
        MessageError::InvalidChannel | MessageError::InvalidReply => {
            HttpResponse::BadRequest().body(error.to_string())
        }
        MessageError::Moderation(_) => HttpResponse::BadRequest().body(error.to_string()),
        MessageError::NotFound => HttpResponse::NotFound().body(error.to_string()),
        MessageError::Repository(_) => HttpResponse::InternalServerError().body(error.to_string()),
    };

    response
}