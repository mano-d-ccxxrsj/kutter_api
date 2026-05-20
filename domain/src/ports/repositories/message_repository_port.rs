use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::message_entity::{ChannelMessage, NewChannelMessage};

#[async_trait]
pub trait MessageRepositoryPort: Send + Sync {
    async fn create(&self, message: &NewChannelMessage) -> Result<ChannelMessage, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<ChannelMessage>, RepositoryError>;
    async fn list_by_channel(&self, channel_id: i32) -> Result<Vec<ChannelMessage>, RepositoryError>;
    async fn update_content(&self, id: i32, message: String) -> Result<Option<ChannelMessage>, RepositoryError>;
    async fn delete_by_id(&self, id: i32) -> Result<bool, RepositoryError>;
}