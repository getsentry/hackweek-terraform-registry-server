use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/.well-known/terraform.json")]
async fn service_discovery() -> Result<impl Responder> {
    let services = r#"
        {
            "modules.v1": "/terraform/modules/v1"
        }
    "#;
    let response: Value = serde_json::from_str(services)?;
    Ok(web::Json(response))
}

#[derive(Deserialize, Serialize, Debug)]
struct ModuleAddress {
    namespace: String,
    name: String,
    system: String,
}

#[get("/{namespace}/{name}/{system}")]
async fn module(
    _req: HttpRequest,
    module_address: web::Path<ModuleAddress>,
) -> Result<impl Responder> {
    let path: ModuleAddress = module_address.into_inner();
    Ok(web::Json(path))
}

#[get("/{namespace}/{name}/{system}/versions")]
async fn module_versions(
    _req: HttpRequest,
    module_address: web::Path<ModuleAddress>,
) -> Result<impl Responder> {
    let path: ModuleAddress = module_address.into_inner();
    Ok(web::Json(path))
}

#[derive(Deserialize, Serialize, Debug)]
struct ModuleAddressWithVersion {
    namespace: String,
    name: String,
    system: String,
    version: String,
}

#[get("/{namespace}/{name}/{system}/{version}/download")]
async fn download_module_version(
    _req: HttpRequest,
    module_address: web::Path<ModuleAddressWithVersion>,
) -> Result<impl Responder> {
    let path: ModuleAddressWithVersion = module_address.into_inner();
    Ok(web::Json(path))
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::NormalizePath::default())
            .service(healthz)
            .service(service_discovery)
            .service(module)
            .service(module_versions)
            .service(download_module_version)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
