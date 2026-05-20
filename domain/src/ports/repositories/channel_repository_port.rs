use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::channel_entity::{Channel, NewChannel};

#[async_trait]
pub trait ChannelRepositoryPort: Send + Sync {
    async fn create(&self, channel: &NewChannel) -> Result<Channel, RepositoryError>;
    async fn belongs_to_community(&self, channel_id: i32, community_id: i32) -> Result<bool, RepositoryError>;
}