use actix_files::Files;
use actix_web::{get, middleware::Logger, App, HttpServer};

use terraform_registry_server::handlers::{
    download_module_version, healthz, module, module_versions, service_discovery,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8000");

    HttpServer::new(|| {
        App::new()
            .service(healthz)
            .service(service_discovery)
            .service(module)
            .service(module_versions)
            .service(download_module_version)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
