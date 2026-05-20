use async_trait::async_trait;

use crate::entities::channel_entity::{Channel, NewChannel};
use crate::errors::channel_error::ChannelError;
use crate::ports::repositories::channel_repository_port::ChannelRepositoryPort;
use crate::ports::repositories::member_repository_port::MemberRepositoryPort;
use crate::ports::services::channel_service_port::ChannelServicePort;
use crate::services::types::ChannelService;
use crate::types::channel_types::CreateChannelCommand;

impl<ChannelRepository, MemberRepository> ChannelService<ChannelRepository, MemberRepository>
where
    ChannelRepository: ChannelRepositoryPort,
    MemberRepository: MemberRepositoryPort,
{
    pub fn new(channels: ChannelRepository, members: MemberRepository) -> Self {
        Self { channels, members }
    }
}

#[async_trait]
impl<ChannelRepository, MemberRepository> ChannelServicePort
    for ChannelService<ChannelRepository, MemberRepository>
where
    ChannelRepository: ChannelRepositoryPort,
    MemberRepository: MemberRepositoryPort,
{
    async fn create(&self, command: CreateChannelCommand) -> Result<Channel, ChannelError> {
        let is_member: bool = self
            .members
            .is_member(command.user_id, command.community_id)
            .await?;

        if !is_member {
            return Err(ChannelError::Unauthorized);
        }

        let channel: NewChannel = NewChannel {
            community_id: command.community_id,
            name: command.name,
            topic: command.topic,
            hidden: command.hidden,
        };

        let created: Channel = self.channels.create(&channel).await?;

        Ok(created)
    }
}