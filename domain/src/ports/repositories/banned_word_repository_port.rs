use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::banned_word_entity::BannedWord;

#[async_trait]
pub trait BannedWordRepositoryPort: Send + Sync {
    async fn find_active(&self) -> Result<Vec<BannedWord>, RepositoryError>;
}