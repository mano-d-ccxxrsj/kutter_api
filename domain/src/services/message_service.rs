use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::entities::message_entity::{ChannelMessage, NewChannelMessage};
use crate::errors::message_error::MessageError;
use crate::ports::repositories::channel_repository_port::ChannelRepositoryPort;
use crate::ports::repositories::member_repository_port::MemberRepositoryPort;
use crate::ports::repositories::message_repository_port::MessageRepositoryPort;
use crate::ports::services::message_service_port::MessageServicePort;
use crate::services::types::MessageService;
use crate::types::message_types::{
    DeleteChannelMessageCommand, EditChannelMessageCommand, ListChannelMessagesCommand,
    SendChannelMessageCommand,
};

impl<MessageRepository, MemberRepository, ChannelRepository>
    MessageService<MessageRepository, MemberRepository, ChannelRepository>
where
    MessageRepository: MessageRepositoryPort,
    MemberRepository: MemberRepositoryPort,
    ChannelRepository: ChannelRepositoryPort,
{
    pub fn new(
        messages: MessageRepository,
        members: MemberRepository,
        channels: ChannelRepository,
    ) -> Self {
        Self { messages, members, channels }
    }
}

#[async_trait]
impl<MessageRepository, MemberRepository, ChannelRepository> MessageServicePort
    for MessageService<MessageRepository, MemberRepository, ChannelRepository>
where
    MessageRepository: MessageRepositoryPort,
    MemberRepository: MemberRepositoryPort,
    ChannelRepository: ChannelRepositoryPort,
{
    async fn send(&self, command: SendChannelMessageCommand) -> Result<ChannelMessage, MessageError> {
        let is_member: bool = self
            .members
            .is_member(command.user_id, command.community_id)
            .await?;

        if !is_member {
            return Err(MessageError::Unauthorized);
        }

        let channel_is_valid: bool = self
            .channels
            .belongs_to_community(command.channel_id, command.community_id)
            .await?;

        if !channel_is_valid {
            return Err(MessageError::InvalidChannel);
        }

        if let Some(replied_message_id) = command.replied_message {
            let replied_message: Option<ChannelMessage> = self
                .messages
                .find_by_id(replied_message_id)
                .await?;

            match replied_message {
                Some(message) if message.channel_id == command.channel_id => {}
                _ => return Err(MessageError::InvalidReply),
            }
        }

        let timestamp: DateTime<Utc> = Utc::now();
        let message: NewChannelMessage = NewChannelMessage {
            channel_id: command.channel_id,
            user_id: command.user_id,
            message: command.message,
            replied_message: command.replied_message,
            timestamp,
            edited: false,
        };

        let created: ChannelMessage = self.messages.create(&message).await?;

        Ok(created)
    }

    async fn list(&self, command: ListChannelMessagesCommand) -> Result<Vec<ChannelMessage>, MessageError> {
        let is_member: bool = self
            .members
            .is_member(command.user_id, command.community_id)
            .await?;

        if !is_member {
            return Err(MessageError::Unauthorized);
        }

        let channel_is_valid: bool = self
            .channels
            .belongs_to_community(command.channel_id, command.community_id)
            .await?;

        if !channel_is_valid {
            return Err(MessageError::InvalidChannel);
        }

        let messages: Vec<ChannelMessage> = self
            .messages
            .list_by_channel(command.channel_id)
            .await?;

        Ok(messages)
    }

    async fn edit(&self, command: EditChannelMessageCommand) -> Result<ChannelMessage, MessageError> {
        let current_message: ChannelMessage = match self.messages.find_by_id(command.message_id).await? {
            Some(found) => found,
            None => return Err(MessageError::NotFound),
        };

        if current_message.user_id != command.user_id {
            return Err(MessageError::Unauthorized);
        }

        let updated_message: ChannelMessage = match self
            .messages
            .update_content(command.message_id, command.message)
            .await?
        {
            Some(updated) => updated,
            None => return Err(MessageError::NotFound),
        };

        Ok(updated_message)
    }

    async fn delete(&self, command: DeleteChannelMessageCommand) -> Result<(), MessageError> {
        let current_message: ChannelMessage = match self.messages.find_by_id(command.message_id).await? {
            Some(found) => found,
            None => return Err(MessageError::NotFound),
        };

        if current_message.user_id != command.user_id {
            return Err(MessageError::Unauthorized);
        }

        let deleted: bool = self.messages.delete_by_id(command.message_id).await?;

        if !deleted {
            return Err(MessageError::NotFound);
        }

        Ok(())
    }
}