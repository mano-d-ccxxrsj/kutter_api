use shared::errors::repository_error::RepositoryError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum MessageError {
    Unauthorized,
    InvalidChannel,
    InvalidReply,
    NotFound,
    Repository(RepositoryError),
}

impl Display for MessageError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        match self {
            MessageError::Unauthorized => write!(formatter, "Unauthorized"),
            MessageError::InvalidChannel => write!(formatter, "Channel does not belong to community"),
            MessageError::InvalidReply => write!(formatter, "Cannot reply a message from another channel"),
            MessageError::NotFound => write!(formatter, "Message not found"),
            MessageError::Repository(error) => write!(formatter, "Repository error: {}", error),
        }
    }
}

impl From<RepositoryError> for MessageError {
    fn from(error: RepositoryError) -> Self {
        MessageError::Repository(error)
    }
}

impl std::fmt::Debug for MessageError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(self, formatter)
    }
}