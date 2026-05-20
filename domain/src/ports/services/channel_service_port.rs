use async_trait::async_trait;

use crate::entities::channel_entity::Channel;
use crate::errors::channel_error::ChannelError;
use crate::types::channel_types::CreateChannelCommand;

#[async_trait]
pub trait ChannelServicePort: Send + Sync {
    async fn create(&self, command: CreateChannelCommand) -> Result<Channel, ChannelError>;
}