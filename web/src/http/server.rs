use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self, Data},
};
use domain::ports::services::service_set_port::ServiceSetPort;
use shared::config::types::AppConfig;
use shared::database::ports::PoolPort;

pub async fn run(
    app_config: AppConfig,
    pool: Box<dyn PoolPort>,
    services: Box<dyn ServiceSetPort>,
) -> std::io::Result<()> {
    let pool_data: Data<Box<dyn PoolPort>> = Data::new(pool);
    let services_data: Data<Box<dyn ServiceSetPort>> = Data::new(services);

    let host: String = app_config.app_host;
    let port: u16 = app_config.app_server_port;

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(services_data.clone())
            .service(
                web::scope("/api")
                    .configure(crate::handlers::user_handler::routes)
                    .configure(crate::handlers::message_handler::routes)
                    .configure(crate::handlers::channel_handler::routes)
                    .configure(crate::handlers::community_handler::routes),
            )
    })
    .bind((host.clone(), port))?
    .run();

    server.await
}