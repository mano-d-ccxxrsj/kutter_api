use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::community_entity::{Community, NewCommunity};

#[async_trait]
pub trait CommunityRepositoryPort: Send + Sync {
    async fn create(&self, community: &NewCommunity) -> Result<Community, RepositoryError>;
}