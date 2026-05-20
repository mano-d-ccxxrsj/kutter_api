use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::user_flag_entity::{NewUserFlag, UserFlag};

#[async_trait]
pub trait UserFlagRepositoryPort: Send + Sync {
    async fn create(&self, user_flag: &NewUserFlag) -> Result<UserFlag, RepositoryError>;
}