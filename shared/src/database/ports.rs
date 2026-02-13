use crate::database::aliases::DbFuture;

pub trait PoolPort: Send + Sync {}

pub trait ServicesPort: Send + Sync {}

pub trait DatabasePort: Send + Sync {
    type Pool;
    type Repositories;

    fn create_pool(&self) -> DbFuture<Self::Pool>;
    fn create_repositories(&self, pool: Self::Pool) -> Self::Repositories;
}