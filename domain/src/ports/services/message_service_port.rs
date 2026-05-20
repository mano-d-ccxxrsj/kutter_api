use async_trait::async_trait;

use crate::entities::message_entity::ChannelMessage;
use crate::errors::message_error::MessageError;
use crate::types::message_types::{
    DeleteChannelMessageCommand, EditChannelMessageCommand, ListChannelMessagesCommand,
    SendChannelMessageCommand,
};

#[async_trait]
pub trait MessageServicePort: Send + Sync {
    async fn send(&self, command: SendChannelMessageCommand) -> Result<ChannelMessage, MessageError>;
    async fn list(&self, command: ListChannelMessagesCommand) -> Result<Vec<ChannelMessage>, MessageError>;
    async fn edit(&self, command: EditChannelMessageCommand) -> Result<ChannelMessage, MessageError>;
    async fn delete(&self, command: DeleteChannelMessageCommand) -> Result<(), MessageError>;
}