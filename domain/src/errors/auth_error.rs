use shared::errors::repository_error::RepositoryError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum AuthError {
    EmailAlreadyRegistered,
    UsernameAlreadyRegistered,
    InvalidCredentials,
    Repository(RepositoryError),
    Security(String),
}

impl Display for AuthError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        match self {
            AuthError::EmailAlreadyRegistered => write!(formatter, "Email already registered"),
            AuthError::UsernameAlreadyRegistered => {
                write!(formatter, "Username already registered")
            }
            AuthError::InvalidCredentials => write!(formatter, "Invalid credentials"),
            AuthError::Repository(error) => write!(formatter, "Repository error: {}", error),
            AuthError::Security(error) => write!(formatter, "Security error: {}", error),
        }
    }
}

impl From<RepositoryError> for AuthError {
    fn from(error: RepositoryError) -> Self {
        AuthError::Repository(error)
    }
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(self, formatter)
    }
}