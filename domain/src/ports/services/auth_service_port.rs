use async_trait::async_trait;

use crate::errors::auth_error::AuthError;
use crate::types::auth_types::{
    AuthenticatedSession, LoginSession, LoginUserCommand, RegisterUserCommand,
};

#[async_trait]
pub trait AuthServicePort: Send + Sync {
    async fn register(&self, command: RegisterUserCommand) -> Result<(), AuthError>;
    async fn login(&self, command: LoginUserCommand) -> Result<LoginSession, AuthError>;
    async fn authenticate_session(&self, token: String) -> Result<AuthenticatedSession, AuthError>;
}