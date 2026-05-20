use async_trait::async_trait;
use domain::ports::repositories::token_repository_port::TokenRepositoryPort;
use shared::errors::repository_error::RepositoryError;
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono::Utc;

use crate::database::types::{PoolWrapper, PostgresTokenRepository};

impl PostgresTokenRepository {
    pub fn new(pool: PoolWrapper) -> PostgresTokenRepository {
        PostgresTokenRepository { pool }
    }
}

#[async_trait]
impl TokenRepositoryPort for PostgresTokenRepository {
    async fn create_session(&self, user_id: i32) -> Result<(), RepositoryError> {
        let now: sqlx::types::chrono::DateTime<Utc> = Utc::now();

        let _: PgQueryResult = sqlx::query(
            "INSERT INTO tokens (
                user_id, created_at, last_update
            )
            VALUES (
                $1, $2, $3
            )",
        )
        .bind(user_id)
        .bind(now)
        .bind(now)
        .execute(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        Ok(())
    }
}