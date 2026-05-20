use shared::config::types::AppConfig;
use sqlx::PgPool;

pub struct PoolWrapper {
    pub inner: PgPool,
}

pub struct PostgresDb {
    pub app_config: AppConfig,
}

pub struct PostgresUserRepository {
    pub pool: PoolWrapper,
}

pub struct PostgresTokenRepository {
    pub pool: PoolWrapper,
}

pub struct PostgresChannelRepository {
    pub pool: PoolWrapper,
}

pub struct PostgresCommunityRepository {
    pub pool: PoolWrapper,
}

pub struct PostgresMemberRepository {
    pub pool: PoolWrapper,
}

pub struct PostgresMessageRepository {
    pub pool: PoolWrapper,
}

pub struct RepositorySet {
    pub user_repo: PostgresUserRepository,
    pub token_repo: PostgresTokenRepository,
    pub channel_repo: PostgresChannelRepository,
    pub community_repo: PostgresCommunityRepository,
    pub member_repo: PostgresMemberRepository,
    pub message_repo: PostgresMessageRepository,
}