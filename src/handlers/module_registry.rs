use poem::{
    handler,
    http::StatusCode,
    web::{self, headers::ContentType},
    IntoResponse, Response, Result,
};
use serde::{Deserialize, Serialize};

use std::path::Path;

use crate::configuration::Settings;

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleAddressRequest {
    namespace: String,
    name: String,
    system: String,
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
#[handler]
pub async fn module_versions(
    settings: web::Data<&Settings>,
    web::Path(ModuleAddressRequest {
        namespace,
        name,
        system,
    }): web::Path<ModuleAddressRequest>,
) -> impl IntoResponse {
    let path = Path::new(&settings.root_module_dir)
        .join(&namespace)
        .join(&name)
        .join(&system);

    // check if directory exists
    if !path.exists() {
        return Response::builder().status(StatusCode::NOT_FOUND).finish();
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

    web::Json(module_version_listing).into_response()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleAddressWithVersionRequest {
    pub namespace: String,
    pub name: String,
    pub system: String,
    pub version: String,
}

/// Returns the download link for the requested module's source
/// in the `X-Terraform-Get` header.
#[handler]
pub async fn download_module_version(
    settings: web::Data<&Settings>,
    web::Path(ModuleAddressWithVersionRequest {
        namespace,
        name,
        system,
        version,
    }): web::Path<ModuleAddressWithVersionRequest>,
) -> impl IntoResponse {
    let path = Path::new(&settings.root_module_dir)
        .join(&namespace)
        .join(&name)
        .join(&system)
        .join(&version);

    if !path.exists() {
        return Response::builder().status(StatusCode::NOT_FOUND).finish();
    }

    let download_link = format!(
        "{}/download/{}/{}/{}/{}",
        &settings.base_url, namespace, name, system, version
    );

    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("X-Terraform-Get", download_link)
        .finish()
}

#[handler]
pub async fn download_latest_module_version(
    settings: web::Data<&Settings>,
    web::Path(ModuleAddressRequest {
        namespace,
        name,
        system,
    }): web::Path<ModuleAddressRequest>,
) -> impl IntoResponse {
    // should return a 302 to the download URL of the latest version available
    ()
}
