use domain::ports::services::service_set_port::ServiceSetPort;
use domain::services::types::{
    AuthService, ChannelService, CommunityService, MessageService, ProfanityContentModerationService,
    ToggleContentModerationService, WebServiceSet,
};
use persistence::database::types::{
    PoolWrapper, PostgresBannedWordRepository, PostgresChannelRepository,
    PostgresCommunityRepository, PostgresDb, PostgresMemberRepository, PostgresMessageRepository,
    PostgresTokenRepository, PostgresUserFlagRepository, PostgresUserRepository, RepositorySet,
};
use security::jwt::token_service::JwtSessionTokenService;
use security::keys::public_key_service::X25519PublicKeyGenerator;
use security::password::hash_service::BcryptPasswordHash;
use shared::config::types::AppConfig;
use shared::database::ports::{DatabasePort, PoolPort};

pub struct ApplicationConfig;

impl ApplicationConfig {
    pub fn services(config: &AppConfig, repositories: RepositorySet) -> WebServiceSet {
        let auth_service: AuthService<
            PostgresUserRepository,
            PostgresTokenRepository,
            BcryptPasswordHash,
            X25519PublicKeyGenerator,
            JwtSessionTokenService,
        > = AuthService::new(
            repositories.user_repo,
            repositories.token_repo,
            BcryptPasswordHash::new(),
            X25519PublicKeyGenerator::new(),
            JwtSessionTokenService::new(config.jwt_key.clone()),
        );

        let channel_service: ChannelService<
            PostgresChannelRepository,
            PostgresMemberRepository,
        > = ChannelService::new(
            repositories.channel_repo.clone(),
            repositories.member_repo.clone(),
        );

        let community_service: CommunityService<
            PostgresCommunityRepository,
            PostgresMemberRepository,
        > = CommunityService::new(
            repositories.community_repo,
            repositories.member_repo.clone(),
        );

        let message_service: MessageService<
            PostgresMessageRepository,
            PostgresMemberRepository,
            PostgresChannelRepository,
            ToggleContentModerationService<
                ProfanityContentModerationService<
                    PostgresBannedWordRepository,
                    PostgresUserFlagRepository,
                >,
            >,
        > = MessageService::new(
            repositories.message_repo,
            repositories.member_repo,
            repositories.channel_repo,
            Self::content_moderation_service(
                config,
                repositories.banned_word_repo,
                repositories.user_flag_repo,
            ),
        );

        let services: WebServiceSet = WebServiceSet::new(
            Box::new(auth_service),
            Box::new(channel_service),
            Box::new(community_service),
            Box::new(message_service),
        );

        services
    }

    pub fn content_moderation_service(
        config: &AppConfig,
        banned_word_repository: PostgresBannedWordRepository,
        user_flag_repository: PostgresUserFlagRepository,
    ) -> ToggleContentModerationService<
        ProfanityContentModerationService<PostgresBannedWordRepository, PostgresUserFlagRepository>,
    > {
        let profanity_service: ProfanityContentModerationService<
            PostgresBannedWordRepository,
            PostgresUserFlagRepository,
        > = ProfanityContentModerationService::new(
            banned_word_repository,
            user_flag_repository,
        );

        let service: ToggleContentModerationService<
            ProfanityContentModerationService<PostgresBannedWordRepository, PostgresUserFlagRepository>,
        > =
            ToggleContentModerationService::new(
                config.content_moderation_enabled,
                profanity_service,
            );

        service
    }

    pub fn pool_port(pool: <PostgresDb as DatabasePort>::Pool) -> Box<dyn PoolPort> {
        let pool_wrapper: PoolWrapper = PoolWrapper { inner: pool };
        let pool_port: Box<dyn PoolPort> = Box::new(pool_wrapper);

        pool_port
    }

    pub fn service_set(services: WebServiceSet) -> Box<dyn ServiceSetPort> {
        let service_set: Box<dyn ServiceSetPort> = Box::new(services);

        service_set
    }
}