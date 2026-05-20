use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, web};
use domain::entities::community_entity::Community;
use domain::errors::auth_error::AuthError;
use domain::errors::community_error::CommunityError;
use domain::ports::services::service_set_port::ServiceSetPort;
use domain::types::auth_types::AuthenticatedSession;
use domain::types::community_types::CreateCommunityCommand;

use crate::dto::request::community_request_dto::CreateCommunityRequestDto;
use crate::dto::request::types::DeserializeManual;
use crate::dto::response::community_response_dto::CommunityResponseDto;
use crate::dto::response::types::SerializeManual;
use crate::handlers::request_body::RequestBody;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/community")
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

    let dto: CreateCommunityRequestDto = match CreateCommunityRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: CreateCommunityCommand = CreateCommunityCommand {
        owner_user_id: session.user_id,
        name: dto.name,
        about: dto.about,
        nsfw: dto.nsfw,
    };

    let response: HttpResponse = match services.community_service().create(command).await {
        Ok(community) => {
            let created: Community = community;
            let dto: CommunityResponseDto = created.into();

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(dto.serialize_json())
        }
        Err(error) => community_error_response(error),
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

fn community_error_response(error: CommunityError) -> HttpResponse {
    let response: HttpResponse = match error {
        CommunityError::Repository(_) => HttpResponse::InternalServerError().body(error.to_string()),
    };

    response
}