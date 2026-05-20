use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::entities::user_entity::{NewUser, User};
use crate::errors::auth_error::AuthError;
use crate::ports::repositories::token_repository_port::TokenRepositoryPort;
use crate::ports::repositories::user_repository_port::UserRepositoryPort;
use crate::ports::security::password_hash_port::PasswordHashPort;
use crate::ports::security::public_key_generator_port::PublicKeyGeneratorPort;
use crate::ports::security::session_token_port::SessionTokenPort;
use crate::ports::services::auth_service_port::AuthServicePort;
use crate::services::types::AuthService;
use crate::types::auth_types::{
    AuthenticatedSession, LoginSession, LoginUserCommand, RegisterUserCommand,
};

impl<UserRepository, TokenRepository, PasswordHash, PublicKeyGenerator, SessionToken>
    AuthService<UserRepository, TokenRepository, PasswordHash, PublicKeyGenerator, SessionToken>
where
    UserRepository: UserRepositoryPort,
    TokenRepository: TokenRepositoryPort,
    PasswordHash: PasswordHashPort,
    PublicKeyGenerator: PublicKeyGeneratorPort,
    SessionToken: SessionTokenPort,
{
    pub fn new(
        users: UserRepository,
        tokens: TokenRepository,
        password_hash: PasswordHash,
        public_keys: PublicKeyGenerator,
        session_tokens: SessionToken,
    ) -> Self {
        Self {
            users,
            tokens,
            password_hash,
            public_keys,
            session_tokens,
        }
    }
}

#[async_trait]
impl<UserRepository, TokenRepository, PasswordHash, PublicKeyGenerator, SessionToken>
    AuthServicePort
    for AuthService<UserRepository, TokenRepository, PasswordHash, PublicKeyGenerator, SessionToken>
where
    UserRepository: UserRepositoryPort,
    TokenRepository: TokenRepositoryPort,
    PasswordHash: PasswordHashPort,
    PublicKeyGenerator: PublicKeyGeneratorPort,
    SessionToken: SessionTokenPort,
{
    async fn register(&self, command: RegisterUserCommand) -> Result<(), AuthError> {
        if self.users.find_by_email(&command.email).await?.is_some() {
            return Err(AuthError::EmailAlreadyRegistered);
        }

        if self
            .users
            .find_by_username(&command.username)
            .await?
            .is_some()
        {
            return Err(AuthError::UsernameAlreadyRegistered);
        }

        let now: DateTime<Utc> = Utc::now();
        let password_hash: String = self
            .password_hash
            .hash_password(&command.password)
            .map_err(AuthError::Security)?;

        let user: NewUser = NewUser {
            username: command.username,
            email: command.email,
            password_hash,
            public_key: self.public_keys.generate_public_key(),
            verified: false,
            profile_picture: None,
            biography: None,
            created_at: now,
            updated_at: now,
        };

        self.users.create(&user).await?;

        Ok(())
    }

    async fn login(&self, command: LoginUserCommand) -> Result<LoginSession, AuthError> {
        let user: User = self
            .users
            .find_by_email(&command.email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        let password_is_valid: bool = self
            .password_hash
            .verify_password(&command.password, &user.password_hash)
            .map_err(AuthError::Security)?;

        if !password_is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        self.tokens.create_session(user.id).await?;

        let session_token: String = self
            .session_tokens
            .generate_session_token(user.id)
            .map_err(AuthError::Security)?;

        Ok(LoginSession { session_token })
    }

    async fn authenticate_session(&self, token: String) -> Result<AuthenticatedSession, AuthError> {
        let user_id: i32 = self
            .session_tokens
            .verify_session_token(&token)
            .map_err(AuthError::Security)?;

        let _: User = self
            .users
            .find_by_id(user_id)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        Ok(AuthenticatedSession { user_id })
    }
}