use async_trait::async_trait;
use domain::entities::message_entity::{ChannelMessage, NewChannelMessage};
use domain::ports::repositories::message_repository_port::MessageRepositoryPort;
use shared::errors::repository_error::RepositoryError;
use sqlx::Error;

use crate::database::types::{PoolWrapper, PostgresMessageRepository};
use crate::mappers::message_mapper::MessageMapper;
use crate::models::message_model::MessageModel;

impl PostgresMessageRepository {
    pub fn new(pool: PoolWrapper) -> PostgresMessageRepository {
        PostgresMessageRepository { pool }
    }
}

#[async_trait]
impl MessageRepositoryPort for PostgresMessageRepository {
    async fn create(&self, message: &NewChannelMessage) -> Result<ChannelMessage, RepositoryError> {
        let model: MessageModel = sqlx::query_as::<_, MessageModel>(
            "INSERT INTO messages (
                channel_id, user_id, message, replied_message, timestamp, edited
            )
            VALUES (
                $1, $2, $3, $4, $5, $6
            )
            RETURNING id, channel_id, user_id, message, replied_message, timestamp, edited",
        )
        .bind(message.channel_id)
        .bind(message.user_id)
        .bind(&message.message)
        .bind(message.replied_message)
        .bind(message.timestamp)
        .bind(message.edited)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let entity: ChannelMessage = MessageMapper::from_model(model);

        Ok(entity)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<ChannelMessage>, RepositoryError> {
        let model: Option<MessageModel> = sqlx::query_as::<_, MessageModel>(
            "SELECT id, channel_id, user_id, message, replied_message, timestamp, edited
            FROM messages
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let entity: Option<ChannelMessage> = model.map(MessageMapper::from_model);

        Ok(entity)
    }

    async fn list_by_channel(&self, channel_id: i32) -> Result<Vec<ChannelMessage>, RepositoryError> {
        let models: Vec<MessageModel> = sqlx::query_as::<_, MessageModel>(
            "SELECT id, channel_id, user_id, message, replied_message, timestamp, edited
            FROM messages
            WHERE channel_id = $1
            ORDER BY timestamp ASC, id ASC",
        )
        .bind(channel_id)
        .fetch_all(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let messages: Vec<ChannelMessage> = models
            .into_iter()
            .map(MessageMapper::from_model)
            .collect();

        Ok(messages)
    }

    async fn update_content(&self, id: i32, message: String) -> Result<Option<ChannelMessage>, RepositoryError> {
        let model: Option<MessageModel> = sqlx::query_as::<_, MessageModel>(
            "UPDATE messages
            SET message = $1, edited = TRUE
            WHERE id = $2
            RETURNING id, channel_id, user_id, message, replied_message, timestamp, edited",
        )
        .bind(message)
        .bind(id)
        .fetch_optional(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let entity: Option<ChannelMessage> = model.map(MessageMapper::from_model);

        Ok(entity)
    }

    async fn delete_by_id(&self, id: i32) -> Result<bool, RepositoryError> {
        let _: sqlx::postgres::PgQueryResult = sqlx::query(
            "UPDATE messages
            SET replied_message = NULL
            WHERE replied_message = $1",
        )
        .bind(id)
        .execute(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let result: sqlx::postgres::PgQueryResult = sqlx::query(
            "DELETE FROM messages
            WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        let deleted: bool = result.rows_affected() > 0;

        Ok(deleted)
    }
}