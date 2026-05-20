use crate::ports::repositories::channel_repository_port::ChannelRepositoryPort;
use crate::ports::repositories::community_repository_port::CommunityRepositoryPort;
use crate::ports::repositories::member_repository_port::MemberRepositoryPort;
use crate::ports::repositories::message_repository_port::MessageRepositoryPort;
use crate::ports::repositories::token_repository_port::TokenRepositoryPort;
use crate::ports::repositories::user_repository_port::UserRepositoryPort;
use crate::ports::security::password_hash_port::PasswordHashPort;
use crate::ports::security::public_key_generator_port::PublicKeyGeneratorPort;
use crate::ports::security::session_token_port::SessionTokenPort;
use crate::ports::services::auth_service_port::AuthServicePort;
use crate::ports::services::channel_service_port::ChannelServicePort;
use crate::ports::services::community_service_port::CommunityServicePort;
use crate::ports::services::message_service_port::MessageServicePort;

pub struct AuthService<
    UserRepository,
    TokenRepository,
    PasswordHash,
    PublicKeyGenerator,
    SessionToken,
> where
    UserRepository: UserRepositoryPort,
    TokenRepository: TokenRepositoryPort,
    PasswordHash: PasswordHashPort,
    PublicKeyGenerator: PublicKeyGeneratorPort,
    SessionToken: SessionTokenPort,
{
    pub users: UserRepository,
    pub tokens: TokenRepository,
    pub password_hash: PasswordHash,
    pub public_keys: PublicKeyGenerator,
    pub session_tokens: SessionToken,
}

pub struct WebServiceSet {
    pub auth_service: Box<dyn AuthServicePort>,
    pub channel_service: Box<dyn ChannelServicePort>,
    pub community_service: Box<dyn CommunityServicePort>,
    pub message_service: Box<dyn MessageServicePort>,
}

pub struct ChannelService<ChannelRepository, MemberRepository>
where
    ChannelRepository: ChannelRepositoryPort,
    MemberRepository: MemberRepositoryPort,
{
    pub channels: ChannelRepository,
    pub members: MemberRepository,
}

pub struct CommunityService<CommunityRepository, MemberRepository>
where
    CommunityRepository: CommunityRepositoryPort,
    MemberRepository: MemberRepositoryPort,
{
    pub communities: CommunityRepository,
    pub members: MemberRepository,
}

pub struct MessageService<MessageRepository, MemberRepository, ChannelRepository>
where
    MessageRepository: MessageRepositoryPort,
    MemberRepository: MemberRepositoryPort,
    ChannelRepository: ChannelRepositoryPort,
{
    pub messages: MessageRepository,
    pub members: MemberRepository,
    pub channels: ChannelRepository,
}