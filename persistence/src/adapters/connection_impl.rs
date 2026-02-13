use crate::database::types::{PoolWrapper, PostgresDb, RepositorySet};
use shared::config::types::AppConfig;
use shared::database::aliases::DbFuture;
use shared::database::ports::{DatabasePort, PoolPort};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::time::Duration;

impl PoolPort for PoolWrapper {}

impl PostgresDb {
    pub fn new(app_config: AppConfig) -> PostgresDb {
        PostgresDb { app_config }
    }
}

impl DatabasePort for PostgresDb {
    type Pool = PgPool;
    type Repositories = RepositorySet;

    fn create_pool(&self) -> DbFuture<Self::Pool> {
        let config: AppConfig = self.app_config.clone();

        Box::pin(async move {
            PgPoolOptions::new()
                .max_connections(config.db_max_connections)
                .min_connections(config.db_min_connections)
                .acquire_timeout(Duration::from_secs(config.db_acquire_timeout))
                .idle_timeout(Duration::from_secs(config.db_idle_timeout))
                .max_lifetime(Duration::from_secs(config.db_max_lifetime))
                .connect(&config.database_url)
                .await
                .map_err(|e:Error | e.to_string())
        })
    }

    fn create_repositories(&self, pool: Self::Pool) -> Self::Repositories {
        todo!()
    }
}