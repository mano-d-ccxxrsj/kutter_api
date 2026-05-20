use async_trait::async_trait;
use domain::entities::member_entity::{Member, NewMember};
use domain::ports::repositories::member_repository_port::MemberRepositoryPort;
use shared::errors::repository_error::RepositoryError;

use crate::database::types::{PoolWrapper, PostgresMemberRepository};
use crate::mappers::member_mapper::MemberMapper;
use crate::models::member_model::MemberModel;

impl PostgresMemberRepository {
    pub fn new(pool: PoolWrapper) -> PostgresMemberRepository {
        PostgresMemberRepository { pool }
    }
}

#[async_trait]
impl MemberRepositoryPort for PostgresMemberRepository {
    async fn join(&self, member: &NewMember) -> Result<Member, RepositoryError> {
        let model: MemberModel = sqlx::query_as::<_, MemberModel>(
            "INSERT INTO members (
                user_id, community_id, owner, admin
            )
            VALUES (
                $1, $2, $3, $4
            )
            RETURNING id, user_id, community_id, owner, admin",
        )
        .bind(member.user_id)
        .bind(member.community_id)
        .bind(member.owner)
        .bind(member.admin)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        let entity: Member = MemberMapper::from_model(model);

        Ok(entity)
    }

    async fn is_member(&self, user_id: i32, community_id: i32) -> Result<bool, RepositoryError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS (
                SELECT 1 FROM members
                WHERE user_id = $1 AND community_id = $2
            )",
        )
        .bind(user_id)
        .bind(community_id)
        .fetch_one(&self.pool.inner)
        .await
        .map_err(|error| RepositoryError::new(error.to_string()))?;

        Ok(result.0)
    }
}