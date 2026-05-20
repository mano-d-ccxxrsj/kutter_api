use async_trait::async_trait;
use domain::entities::channel_entity::{Channel, NewChannel};
use domain::ports::repositories::channel_repository_port::ChannelRepositoryPort;
use shared::errors::repository_error::RepositoryError;
use sqlx::Error;

use crate::database::types::{PoolWrapper, PostgresChannelRepository};
use crate::mappers::channel_mapper::ChannelMapper;
use crate::models::channel_model::ChannelModel;

impl PostgresChannelRepository {
    pub fn new(pool: PoolWrapper) -> PostgresChannelRepository {
        PostgresChannelRepository { pool }
    }
}

#[async_trait]
impl ChannelRepositoryPort for PostgresChannelRepository {
    async fn create(&self, channel: &NewChannel) -> Result<Channel, RepositoryError> {
        let model: ChannelModel = sqlx::query_as::<_, ChannelModel>(
            "INSERT INTO channels (
                community_id, name, topic, hidden
            )
            VALUES (
                $1, $2, $3, $4
            )
            RETURNING id, community_id, name, topic, hidden",
        )
        .bind(channel.community_id)
        .bind(&channel.name)
        .bind(&channel.topic)
        .bind(channel.hidden)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        let entity: Channel = ChannelMapper::from_model(model);

        Ok(entity)
    }

    async fn belongs_to_community(&self, channel_id: i32, community_id: i32) -> Result<bool, RepositoryError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS (
                SELECT 1 FROM channels
                WHERE id = $1 AND community_id = $2
            )",
        )
        .bind(channel_id)
        .bind(community_id)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error: Error| RepositoryError::new(error.to_string()))?;

        Ok(result.0)
    }
}