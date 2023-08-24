use poem::{get, listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use terraform_registry_server::handlers::{
    download_module_version, healthz, module_versions, service_discovery,
};

use terraform_registry_server::configuration;

fn module_routes() -> Route {
    Route::new()
        .at("/:namespace/:name/:system/versions", get(module_versions))
        .at(
            "/:namespace/:name/:system/:version/download",
            get(download_module_version),
        )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let settings = configuration::get_configuration().unwrap();

    log::info!(
        "starting HTTP server at {}:{}",
        &settings.host,
        &settings.port
    );

    let app = Route::new()
        .at("/healthz", get(healthz))
        .at("/.well-known/terraform.json", get(service_discovery))
        .nest("/v1/modules", module_routes())
        .with(AddData::new(settings.clone()));

    Server::new(TcpListener::bind((settings.host, settings.port)))
        .run(app)
        .await
}
