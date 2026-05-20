use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

#[async_trait]
pub trait TokenRepositoryPort: Send + Sync {
    async fn create_session(&self, user_id: i32) -> Result<(), RepositoryError>;
}