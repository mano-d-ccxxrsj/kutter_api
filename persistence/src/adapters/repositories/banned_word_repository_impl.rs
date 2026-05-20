use async_trait::async_trait;
use domain::entities::banned_word_entity::BannedWord;
use domain::ports::repositories::banned_word_repository_port::BannedWordRepositoryPort;
use shared::errors::repository_error::RepositoryError;
use sqlx::Error;

use crate::database::types::{PoolWrapper, PostgresBannedWordRepository};
use crate::mappers::banned_word_mapper::BannedWordMapper;
use crate::models::banned_word_model::BannedWordModel;

impl PostgresBannedWordRepository {
    pub fn new(pool: PoolWrapper) -> PostgresBannedWordRepository {
        PostgresBannedWordRepository { pool }
    }
}

#[async_trait]
impl BannedWordRepositoryPort for PostgresBannedWordRepository {
    async fn find_active(&self) -> Result<Vec<BannedWord>, RepositoryError> {
        let models: Vec<BannedWordModel> = sqlx::query_as::<_, BannedWordModel>(
            "SELECT id, word, active
            FROM banned_words
            WHERE active = TRUE
            ORDER BY word ASC",
        )
        .fetch_all(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let banned_words: Vec<BannedWord> = models
            .into_iter()
            .map(BannedWordMapper::from_model)
            .collect();

        Ok(banned_words)
    }
}