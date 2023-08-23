use actix_web::{
    get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder, Result,
};
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

/// Returns a JSON response of all available versions for the
/// requested module [namespace](https://developer.hashicorp.com/terraform/internals/module-registry-protocol#namespace-1),
/// [name](https://developer.hashicorp.com/terraform/internals/module-registry-protocol#name-1),
/// and [system](https://developer.hashicorp.com/terraform/internals/module-registry-protocol#system-1).
///
/// Example JSON response:
///
/// ```json
/// {
///  "modules": [
///      {
///          "versions": [
///              {"version": "1.0.0"},
///              {"version": "..."}
///          ]
///      }
///  ]
/// }
/// ```
///
/// # Panics
///
/// - Panics if the directory where modules are stored is not readable.
/// - Panics if there are no versions available for the specific module path.
#[get("/{namespace}/{name}/{system}/versions")]
pub async fn module_versions(
    _req: HttpRequest,
    state: web::Data<AppState>,
    module_address: web::Path<ModuleAddressRequest>,
) -> HttpResponse {
    let module_address: ModuleAddressRequest = module_address.into_inner();

    let path = Path::new(&state.settings.root_module_dir)
        .join(&module_address.namespace)
        .join(&module_address.name)
        .join(&module_address.system);

    // check if directory exists
    if !path.exists() {
        return HttpResponse::NotFound().finish();
    }

    // {root_module_dir}/{namespace}/{name}/{system}/1.0.0
    // TODO: remove all the unwraps and handle no versions gracefully
    let mut versions: Vec<Version> = vec![];
    for path in path.read_dir().expect("failed to read module directory") {
        if path.is_ok() {
            let version = Version {
                version: path.unwrap().file_name().into_string().unwrap(),
            };
            versions.push(version);
        }
    }

    let module_versions: Vec<Module> = vec![Module { versions }];
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

/// Returns the download link for the requested module's source
/// in the `X-Terraform-Get` header.
#[get("/{namespace}/{name}/{system}/{version}/download")]
pub async fn download_module_version(
    _req: HttpRequest,
    state: web::Data<AppState>,
    module_address: web::Path<ModuleAddressWithVersionRequest>,
) -> HttpResponse {
    let module_address_with_version: ModuleAddressWithVersionRequest = module_address.into_inner();

    let path = Path::new(&state.settings.root_module_dir)
        .join(&module_address_with_version.namespace)
        .join(&module_address_with_version.name)
        .join(&module_address_with_version.system)
        .join(&module_address_with_version.version);

    if !path.exists() {
        return HttpResponse::NotFound().finish();
    }

    let download_link = format!(
        "{}/download/{}/{}/{}/{}",
        &state.settings.base_url,
        module_address_with_version.namespace,
        module_address_with_version.name,
        module_address_with_version.system,
        module_address_with_version.version
    );

    let response = HttpResponse::NoContent()
        .insert_header(("X-Terraform-Get", download_link))
        .finish();

    response
}

#[get("/download/{namespace}/{name}/{system}/{version}")]
pub async fn download(_req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {
    todo!()
}
