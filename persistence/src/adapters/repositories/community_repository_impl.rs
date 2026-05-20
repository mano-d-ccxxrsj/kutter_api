use async_trait::async_trait;
use domain::entities::community_entity::{Community, NewCommunity};
use domain::ports::repositories::community_repository_port::CommunityRepositoryPort;
use shared::errors::repository_error::RepositoryError;

use crate::database::types::{PoolWrapper, PostgresCommunityRepository};
use crate::mappers::community_mapper::CommunityMapper;
use crate::models::community_model::CommunityModel;

impl PostgresCommunityRepository {
    pub fn new(pool: PoolWrapper) -> PostgresCommunityRepository {
        PostgresCommunityRepository { pool }
    }
}

#[async_trait]
impl CommunityRepositoryPort for PostgresCommunityRepository {
    async fn create(&self, community: &NewCommunity) -> Result<Community, RepositoryError> {
        let model: CommunityModel = sqlx::query_as::<_, CommunityModel>(
            "INSERT INTO communities (
                name, about, nsfw
            )
            VALUES (
                $1, $2, $3
            )
            RETURNING id, name, about, nsfw",
        )
        .bind(&community.name)
        .bind(&community.about)
        .bind(community.nsfw)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        let entity: Community = CommunityMapper::from_model(model);

        Ok(entity)
    }
}