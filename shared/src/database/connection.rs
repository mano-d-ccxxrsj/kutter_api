pub trait PoolPort: Send + Sync {}

pub trait DatabasePort: Send + Sync {
    type Pool;
    type Repositories;

    fn create_pool(&self) -> impl Future<Output = Self::Pool> + Send;
    fn create_repositories(&self, pool: Self::Pool) -> Self::Repositories;
}