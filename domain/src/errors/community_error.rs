use shared::errors::repository_error::RepositoryError;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum CommunityError {
    Repository(RepositoryError),
}

impl Display for CommunityError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        match self {
            CommunityError::Repository(error) => write!(formatter, "Repository error: {}", error),
        }
    }
}

impl From<RepositoryError> for CommunityError {
    fn from(error: RepositoryError) -> Self {
        CommunityError::Repository(error)
    }
}

impl std::fmt::Debug for CommunityError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(self, formatter)
    }
}