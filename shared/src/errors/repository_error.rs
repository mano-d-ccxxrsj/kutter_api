use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct RepositoryError {
    message: String,
}

impl RepositoryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for RepositoryError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{}", self.message)
    }
}

impl std::fmt::Debug for RepositoryError {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        Display::fmt(self, formatter)
    }
}

impl Error for RepositoryError {}