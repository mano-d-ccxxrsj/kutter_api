use sqlx::PgPool;
use shared::config::types::AppConfig;

pub struct PoolWrapper {
    pub inner: PgPool,
}

pub struct PostgresDb {
    pub app_config: AppConfig,
}

pub struct PostgresUserRepository {
    pub pool: PoolWrapper,
}

pub struct PostgresFriendRepository {
    pub pool: PoolWrapper,
}

pub struct RepositorySet {
    pub user_repo: PostgresUserRepository,
    pub friend_repo: PostgresFriendRepository,
}