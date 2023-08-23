use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result, HttpResponseBuilder, http::header::ContentType};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::state::AppState;

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

// {
//  "modules": [
//      {
//          "versions": [
//              {"version": "1.0.0"},
//              {"version": "..."}
//          ]
//      }
//  ]
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleVersionListResponse {
    modules: Vec<Module>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Module {
    versions: Vec<Version>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Version {
    version: String,
}

#[get("/{namespace}/{name}/{system}/versions")]
pub async fn module_versions(
    _req: HttpRequest,
    state: web::Data<AppState>,
    module_address: web::Path<ModuleAddressRequest>,
) -> HttpResponse {
    let module_address: ModuleAddressRequest = module_address.into_inner();

    let path = Path::new(&state.root_module_dir)
        .join(&module_address.namespace)
        .join(&module_address.name)
        .join(&module_address.system);

    // check if directory exists
    if path.exists() == false {
        return HttpResponse::NotFound().finish();
    }

    // {root_module_dir}/{namespace}/{name}/{system}/1.0.0
    // TODO: remove all the unwraps and handle no versions gracefully
    let mut versions: Vec<Version> = vec![];
    for path in path.read_dir().unwrap() {
        if path.is_ok() {
            let version = Version {
                version: path.unwrap().file_name().into_string().unwrap(),
            };
            versions.push(version);
        }
    }

    let mut module_versions: Vec<Module> = vec![];
    module_versions.push(Module { versions });
    let module_version_listing = ModuleVersionListResponse {
        modules: module_versions,
    };


    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(module_version_listing)
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
