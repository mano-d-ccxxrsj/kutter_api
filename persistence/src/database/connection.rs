use std::time::Duration;
use sqlx::{Error, PgPool, Pool, Postgres, postgres::PgPoolOptions};
use shared::config::types::AppConfig;
use shared::database::connection::DatabasePort;
use shared::database::types::DbFuture;
use crate::database::types::{PostgresDb, RepositorySet};
use crate::entity::user::UserRepository;

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

    async fn health_check(&self, pool: &Self::Pool) -> DbFuture<bool> {
        let pool_clone: Pool<Postgres> = pool.clone();
        Box::pin(async move {
            sqlx::query("SELECT 1")
                .execute(&pool_clone)
                .await
                .map(|_| true)
                .map_err(|e: Error| e.to_string())
        })
    }

    fn create_repositories(&self, pool: Self::Pool) -> Self::Repositories {
        RepositorySet {
            user: UserRepository::new(pool),
        }
    }
}