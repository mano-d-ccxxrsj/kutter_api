use sqlx::PgPool;

pub struct UserRepository {
    pub pool: PgPool,
}