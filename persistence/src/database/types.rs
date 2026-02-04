use sqlx::PgPool;
use shared::config::types::AppConfig;
use crate::entity::models::{ChatMessageRepository, FriendRepository, UserRepository};

pub struct PoolWrapper {
    pub inner: PgPool,
}

pub struct PostgresDb {
    pub config: AppConfig,
}

pub struct RepositorySet {
    pub user_repo: UserRepository,
    pub chat_message_repo: ChatMessageRepository,
    pub friend_repo: FriendRepository,
}