pub struct RegisterUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

pub struct LoginSession {
    pub session_token: String,
}

pub struct AuthenticatedSession {
    pub user_id: i32,
}