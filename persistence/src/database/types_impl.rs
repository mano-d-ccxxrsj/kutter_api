use crate::database::types::{PoolWrapper, PostgresChannelRepository, PostgresMemberRepository};

impl Clone for PoolWrapper {
    fn clone(&self) -> PoolWrapper {
        PoolWrapper {
            inner: self.inner.clone(),
        }
    }
}

impl Clone for PostgresMemberRepository {
    fn clone(&self) -> PostgresMemberRepository {
        PostgresMemberRepository {
            pool: self.pool.clone(),
        }
    }
}

impl Clone for PostgresChannelRepository {
    fn clone(&self) -> PostgresChannelRepository {
        PostgresChannelRepository {
            pool: self.pool.clone(),
        }
    }
}