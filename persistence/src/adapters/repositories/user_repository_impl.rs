use crate::database::types::{PoolWrapper, PostgresUserRepository};

impl PostgresUserRepository {
    pub fn new(pool: PoolWrapper) -> PostgresUserRepository {
        PostgresUserRepository { pool }
    }
}