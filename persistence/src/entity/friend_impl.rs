use sqlx::PgPool;
use crate::entity::models::FriendRepository;

impl FriendRepository {
    pub fn new(pool: PgPool) -> FriendRepository {
        FriendRepository { pool }
    }
}