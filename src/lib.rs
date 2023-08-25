use poem::{
    get,
    middleware::{AddData, Tracing},
    EndpointExt, Route,
};

pub mod configuration;
pub mod handlers;

use crate::handlers::{
    download, download_module_version, healthz, module_versions, service_discovery,
};

fn module_routes() -> Route {
    Route::new()
        .at(":namespace/:name/:system/versions", get(module_versions))
        .at(
            ":namespace/:name/:system/:version/download",
            get(download_module_version),
        )
}

pub fn build_app(
    settings: &configuration::Settings,
) -> poem::middleware::AddDataEndpoint<
    poem::middleware::TracingEndpoint<poem::Route>,
    configuration::Settings,
> {
    Route::new()
        .at("/healthz", get(healthz))
        .at("/.well-known/terraform.json", get(service_discovery))
        .nest("/v1/modules", module_routes())
        .at("/download/:namespace/:name/:system/:version", get(download))
        .with(Tracing)
        .with(AddData::new(settings.clone()))
}
