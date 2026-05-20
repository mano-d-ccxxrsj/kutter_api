use async_trait::async_trait;

use crate::entities::community_entity::{Community, NewCommunity};
use crate::entities::member_entity::{Member, NewMember};
use crate::errors::community_error::CommunityError;
use crate::ports::repositories::community_repository_port::CommunityRepositoryPort;
use crate::ports::repositories::member_repository_port::MemberRepositoryPort;
use crate::ports::services::community_service_port::CommunityServicePort;
use crate::services::types::CommunityService;
use crate::types::community_types::CreateCommunityCommand;

impl<CommunityRepository, MemberRepository> CommunityService<CommunityRepository, MemberRepository>
where
    CommunityRepository: CommunityRepositoryPort,
    MemberRepository: MemberRepositoryPort,
{
    pub fn new(communities: CommunityRepository, members: MemberRepository) -> Self {
        Self { communities, members }
    }
}

#[async_trait]
impl<CommunityRepository, MemberRepository> CommunityServicePort
    for CommunityService<CommunityRepository, MemberRepository>
where
    CommunityRepository: CommunityRepositoryPort,
    MemberRepository: MemberRepositoryPort,
{
    async fn create(&self, command: CreateCommunityCommand) -> Result<Community, CommunityError> {
        let community: NewCommunity = NewCommunity {
            name: command.name,
            about: command.about,
            nsfw: command.nsfw,
        };

        let created: Community = self.communities.create(&community).await?;
        let member: NewMember = NewMember {
            user_id: command.owner_user_id,
            community_id: created.id,
            owner: true,
            admin: true,
        };

        let _: Member = self.members.join(&member).await?;

        Ok(created)
    }
}