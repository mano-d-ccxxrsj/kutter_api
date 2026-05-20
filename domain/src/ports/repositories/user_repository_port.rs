use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::user_entity::{NewUser, User};

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>;
    async fn create(&self, user: &NewUser) -> Result<(), RepositoryError>;
}