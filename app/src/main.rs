use shared::config::ports::{Cloneable, ConfigPort};
use shared::config::types::AppConfig;
use shared::database::connection::DatabasePort;
use infra::config::env::EnvConfig;
use persistence::database::types::PostgresDb;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: AppConfig = <EnvConfig as ConfigPort>::from_env_file();

    let db: PostgresDb = PostgresDb::new(config.clone_config());

    let pool: <PostgresDb as DatabasePort>::Pool = db.create_pool().await;

    let _repos: <PostgresDb as DatabasePort>::Repositories = db.create_repositories(pool.clone());

    Ok(())
}