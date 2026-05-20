use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpResponse, web};
use domain::errors::auth_error::AuthError;
use domain::ports::services::service_set_port::ServiceSetPort;
use domain::types::auth_types::{LoginSession, LoginUserCommand, RegisterUserCommand};

use crate::dto::request::types::DeserializeManual;
use crate::dto::request::user_request_dto::{LoginUserRequestDto, RegisterUserRequestDto};
use crate::handlers::request_body::RequestBody;

pub fn routes(config: &mut web::ServiceConfig) {
    (&mut* config).service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login))),
    );
}

pub async fn register(
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: RegisterUserRequestDto = match RegisterUserRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: RegisterUserCommand = dto.into();
    let response: HttpResponse = match services.auth_service().register(command).await {
        Ok(()) => HttpResponse::Ok().body("registered successfully"),
        Err(error) => auth_error_response(error),
    };

    response
}

pub async fn login(
    services: web::Data<Box<dyn ServiceSetPort>>,
    body: web::Bytes,
) -> HttpResponse {
    let json: String = match RequestBody::string(body) {
        Ok(text) => text,
        Err(response) => return response,
    };

    let dto: LoginUserRequestDto = match LoginUserRequestDto::from_json(&json) {
        Ok(parsed) => parsed,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let command: LoginUserCommand = dto.into();
    let response: HttpResponse = match services.auth_service().login(command).await {
        Ok(session) => {
            let login_session: LoginSession = session;
            HttpResponse::Ok()
                .cookie(session_cookie(login_session.session_token))
                .body("Logged in successfully")
        }
        Err(error) => auth_error_response(error),
    };

    response
}

fn auth_error_response(error: AuthError) -> HttpResponse {
    let response: HttpResponse = match error {
        AuthError::EmailAlreadyRegistered
        | AuthError::UsernameAlreadyRegistered
        | AuthError::InvalidCredentials => HttpResponse::BadRequest().body(error.to_string()),
        AuthError::Repository(_) | AuthError::Security(_) => {
            HttpResponse::InternalServerError().body(error.to_string())
        }
    };

    response
}

fn session_cookie(token: String) -> Cookie<'static> {
    let cookie: Cookie<'static> = Cookie::build("session", token)
        .path("/api")
        .secure(false)
        .same_site(SameSite::Lax)
        .http_only(true)
        .permanent()
        .finish();

    cookie
}