use crate::database::types::{PoolWrapper, PostgresFriendRepository};

impl PostgresFriendRepository {
    pub fn new(pool: PoolWrapper) -> PostgresFriendRepository {
        PostgresFriendRepository { pool }
    }
}