use crate::database::types::DbFuture;

pub trait DatabasePort: Send + Sync {
    type Pool;
    type Repositories;

    fn create_pool(&self) -> impl Future<Output = Self::Pool> + Send;
    fn health_check(&self, pool: &Self::Pool) -> impl Future<Output = DbFuture<bool>> + Send;
    fn create_repositories(&self, pool: Self::Pool) -> Self::Repositories;
}