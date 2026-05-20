use async_trait::async_trait;
use domain::entities::user_flag_entity::{NewUserFlag, UserFlag};
use domain::ports::repositories::user_flag_repository_port::UserFlagRepositoryPort;
use shared::errors::repository_error::RepositoryError;
use sqlx::Error;

use crate::database::types::{PoolWrapper, PostgresUserFlagRepository};
use crate::mappers::user_flag_mapper::UserFlagMapper;
use crate::models::user_flag_model::UserFlagModel;

impl PostgresUserFlagRepository {
    pub fn new(pool: PoolWrapper) -> PostgresUserFlagRepository {
        PostgresUserFlagRepository { pool }
    }
}

#[async_trait]
impl UserFlagRepositoryPort for PostgresUserFlagRepository {
    async fn create(&self, user_flag: &NewUserFlag) -> Result<UserFlag, RepositoryError> {
        let model: UserFlagModel = sqlx::query_as::<_, UserFlagModel>(
            "INSERT INTO user_flags (
                user_id, field, action, target, attempted_text, matched_words, details, created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            RETURNING id, user_id, field, action, target, attempted_text, matched_words, details, created_at",
        )
        .bind(user_flag.user_id)
        .bind(&user_flag.field)
        .bind(&user_flag.action)
        .bind(&user_flag.target)
        .bind(&user_flag.attempted_text)
        .bind(&user_flag.matched_words)
        .bind(&user_flag.details)
        .bind(user_flag.created_at)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let entity: UserFlag = UserFlagMapper::from_model(model);

        Ok(entity)
    }
}