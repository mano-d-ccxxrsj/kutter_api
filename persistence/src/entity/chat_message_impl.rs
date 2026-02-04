use sqlx::PgPool;
use crate::entity::models::ChatMessageRepository;

impl ChatMessageRepository {
    pub fn new(pool: PgPool) -> ChatMessageRepository {
        ChatMessageRepository { pool }
    }
}