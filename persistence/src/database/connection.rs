use crate::database::types::{PoolWrapper, PostgresDb, RepositorySet};
use crate::entity::models::{ChatMessageRepository, FriendRepository, UserRepository};
use shared::config::types::AppConfig;
use shared::database::connection::{DatabasePort, PoolPort};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

impl PoolPort for PoolWrapper {}

impl PostgresDb {
    pub fn new(config: AppConfig) -> PostgresDb {
        PostgresDb { config }
    }
}

impl DatabasePort for PostgresDb {
    type Pool = PgPool;
    type Repositories = RepositorySet;

    async fn create_pool(&self) -> Self::Pool {
        PgPoolOptions::new()
            .max_connections(self.config.db_max_connections)
            .min_connections(self.config.db_min_connections)
            .acquire_timeout(Duration::from_secs(self.config.db_acquire_timeout))
            .idle_timeout(Duration::from_secs(self.config.db_idle_timeout))
            .max_lifetime(Duration::from_secs(self.config.db_max_lifetime))
            .connect(&self.config.database_url)
            .await
            .expect("Falha ao criar pool de conexões com o banco")
    }

    fn create_repositories(&self, pool: Self::Pool) -> Self::Repositories {
        RepositorySet {
            user_repo: UserRepository::new(pool.clone()),
            chat_message_repo: ChatMessageRepository::new(pool.clone()),
            friend_repo: FriendRepository::new(pool.clone()),
        }
    }
}
