use async_trait::async_trait;
use shared::errors::repository_error::RepositoryError;

use crate::entities::member_entity::{Member, NewMember};

#[async_trait]
pub trait MemberRepositoryPort: Send + Sync {
    async fn join(&self, member: &NewMember) -> Result<Member, RepositoryError>;
    async fn is_member(&self, user_id: i32, community_id: i32) -> Result<bool, RepositoryError>;
}