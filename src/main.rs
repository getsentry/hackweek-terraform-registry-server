use actix_web::{middleware::Logger, web, App, HttpServer};

use terraform_registry_server::handlers::{
    download_module_version, healthz, module, module_versions, service_discovery,
};

use terraform_registry_server::configuration;
use terraform_registry_server::state::AppState;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let settings = configuration::get_configuration().unwrap();
    let state = web::Data::new(AppState {
        settings: settings.clone(),
    });

    log::info!(
        "starting HTTP server at {}:{}",
        settings.host,
        settings.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(healthz)
            .service(service_discovery)
            .service(module)
            .service(module_versions)
            .service(download_module_version)
            .wrap(Logger::default())
    })
    .bind((settings.host, settings.port))?
    .run()
    .await
}
