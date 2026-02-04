use shared::config::types::AppConfig;
use crate::entity::user::UserRepository;

pub struct PostgresDb {
    pub config: AppConfig,
}

pub struct RepositorySet {
    pub user: UserRepository,
}