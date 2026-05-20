use shared::errors::repository_error::RepositoryError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum ChannelError {
    Unauthorized,
    Repository(RepositoryError),
}

impl Display for ChannelError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        match self {
            ChannelError::Unauthorized => write!(formatter, "Unauthorized"),
            ChannelError::Repository(error) => write!(formatter, "Repository error: {}", error),
        }
    }
}

impl From<RepositoryError> for ChannelError {
    fn from(error: RepositoryError) -> Self {
        ChannelError::Repository(error)
    }
}

impl std::fmt::Debug for ChannelError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(self, formatter)
    }
}