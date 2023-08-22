use actix_web::{get, web, HttpRequest, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleAddressRequest {
    namespace: String,
    name: String,
    system: String,
}

#[get("/{namespace}/{name}/{system}")]
pub async fn module(
    _req: HttpRequest,
    module_address: web::Path<ModuleAddressRequest>,
) -> Result<impl Responder> {
    let path: ModuleAddressRequest = module_address.into_inner();
    Ok(web::Json(path))
}

#[get("/{namespace}/{name}/{system}/versions")]
pub async fn module_versions(
    _req: HttpRequest,
    module_address: web::Path<ModuleAddressRequest>,
) -> Result<impl Responder> {
    let path: ModuleAddressRequest = module_address.into_inner();
    Ok(web::Json(path))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleAddressWithVersionRequest {
    namespace: String,
    name: String,
    system: String,
    version: String,
}

#[get("/{namespace}/{name}/{system}/{version}/download")]
pub async fn download_module_version(
    _req: HttpRequest,
    module_address: web::Path<ModuleAddressWithVersionRequest>,
) -> Result<impl Responder> {
    let path: ModuleAddressWithVersionRequest = module_address.into_inner();
    Ok(web::Json(path))
}
