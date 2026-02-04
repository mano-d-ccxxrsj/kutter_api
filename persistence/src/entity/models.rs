use sqlx::PgPool;

pub struct UserRepository {
    pub pool: PgPool,
}

pub struct ChatMessageRepository {
    pub pool: PgPool,
}

pub struct FriendRepository {
    pub pool: PgPool,
}