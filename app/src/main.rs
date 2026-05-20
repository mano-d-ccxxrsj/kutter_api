mod config;

use config::application_config::ApplicationConfig;
use domain::services::types::WebServiceSet;
use infra::adapters::types::EnvConfig;
use persistence::database::types::{PostgresDb, RepositorySet};
use shared::config::ports::ConfigPort;
use shared::config::types::AppConfig;
use shared::database::ports::DatabasePort;

fn main() -> std::io::Result<()> {
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(run())
}

async fn run() -> std::io::Result<()> {
    let config: AppConfig = <EnvConfig as ConfigPort>::from_env_file();

    let db: PostgresDb = PostgresDb::new(config.clone());

    let pool: <PostgresDb as DatabasePort>::Pool = db.create_pool().await.expect("Error creating Pool");

    persistence::database::schema::create_auth_schema(&pool).await.expect("Error creating auth schema");
    persistence::database::schema::create_content_moderation_schema(&pool).await.expect("Error creating content moderation schema");
    persistence::database::schema::create_community_schema(&pool).await.expect("Error creating community schema");

    let repositories: RepositorySet = db.create_repositories(pool.clone());

    let services: WebServiceSet = ApplicationConfig::services(&config, repositories);

    web::http::server::run(
        config,
        ApplicationConfig::pool_port(pool),
        ApplicationConfig::service_set(services),
    )
    .await
}