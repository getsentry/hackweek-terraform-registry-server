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
        root_module_dir: settings.root_module_dir,
    });

    log::info!("starting HTTP server at http://localhost:8000");

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
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
