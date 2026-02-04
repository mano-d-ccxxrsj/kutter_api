use sqlx::PgPool;
use crate::entity::models::UserRepository;

impl UserRepository {
    pub fn new(pool: PgPool) -> UserRepository {
        UserRepository { pool }
    }
}