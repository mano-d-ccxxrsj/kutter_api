use async_trait::async_trait;

use crate::entities::community_entity::Community;
use crate::errors::community_error::CommunityError;
use crate::types::community_types::CreateCommunityCommand;

#[async_trait]
pub trait CommunityServicePort: Send + Sync {
    async fn create(&self, command: CreateCommunityCommand) -> Result<Community, CommunityError>;
}