use sqlx::PgPool;
use crate::entity::user::UserRepository;

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}