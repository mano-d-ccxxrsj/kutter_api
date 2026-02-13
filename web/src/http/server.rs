use actix_web::{web::Data, dev::Server, HttpServer, App};
use shared::config::types::AppConfig;
use shared::database::ports::{PoolPort, ServicesPort};

pub async fn run(
    app_config: AppConfig,
    pool: Box<dyn PoolPort>,
    services: Box<dyn ServicesPort>,
) -> std::io::Result<()> {

    let pool_data: Data<Box<dyn PoolPort>> = Data::new(pool);
    let services_data: Data<Box<dyn ServicesPort>> = Data::new(services);

    let host: String = app_config.app_host;
    let port: u16 = app_config.app_server_port;

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(services_data.clone())
    })
        .bind((host.clone(), port))?
        .run();

    println!("Server running at http://{}:{}", host, port);

    server.await
}