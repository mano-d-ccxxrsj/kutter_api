use domain::services::service_set::WebServiceSet;
use infra::adapters::types::EnvConfig;
use persistence::database::types::{PoolWrapper, PostgresDb};
use shared::config::ports::ConfigPort;
use shared::config::types::AppConfig;
use shared::database::ports::DatabasePort;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: AppConfig = <EnvConfig as ConfigPort>::from_env_file();

    let db: PostgresDb = PostgresDb::new(config.clone());

    let pool: <PostgresDb as DatabasePort>::Pool = db.create_pool().await.expect("Error creating Pool");

    let _: <PostgresDb as DatabasePort>::Repositories = db.create_repositories(pool.clone());

    let services: WebServiceSet = WebServiceSet {};

    let pool_wrapper: PoolWrapper = PoolWrapper { inner: pool };

    web::http::server::run(
        config,
        Box::new(pool_wrapper),
        Box::new(services),
    ).await
}