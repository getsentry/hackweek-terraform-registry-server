#![allow(unused_variables)]
use anyhow::{anyhow, Result};
use poem::{
    handler,
    http::StatusCode,
    web::{self},
    IntoResponse, Response,
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
) -> Result<impl IntoResponse> {
    let path = Path::new(&settings.root_module_dir)
        .join(&namespace)
        .join(&name)
        .join(&system);

    // check if directory exists
    if !path.exists() {
        return Ok(Response::builder().status(StatusCode::NOT_FOUND).finish());
    }

    // {root_module_dir}/{namespace}/{name}/{system}/1.0.0
    let mut versions: Vec<Version> = vec![];
    for entry in path.read_dir()? {
        let entry = entry?;
        let version = extract_version(entry)?;
        versions.push(version);
    }

    let module_versions: Vec<Module> = vec![Module { versions }];
    let module_version_listing = ModuleVersionListResponse {
        modules: module_versions,
    };

    Ok(web::Json(module_version_listing).into_response())
}

fn extract_version(entry: std::fs::DirEntry) -> Result<Version> {
    let version_os_str = entry.file_name();
    if let Some(version_str) = &version_os_str.to_str() {
        if let Some(version) = version_str.strip_suffix(".tar.xz") {
            return Ok(Version {
                version: version.to_string(),
            });
        }
    }

    Err(anyhow!("failed to extract version"))
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
) -> Result<impl IntoResponse> {
    let path = Path::new(&settings.root_module_dir)
        .join(&namespace)
        .join(&name)
        .join(&system)
        .join(version.clone() + ".tar.xz"); //TODO: there has to be a cleaner way to do this...

    if !path.exists() {
        return Ok(Response::builder().status(StatusCode::NOT_FOUND).finish());
    }

    let download_link = format!(
        "{}/download/{}/{}/{}/{}.tar.xz",
        &settings.base_url, namespace, name, system, version
    );

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("X-Terraform-Get", download_link)
        .finish())
}

#[handler]
pub async fn download_latest_module_version(
    settings: web::Data<&Settings>,
    web::Path(ModuleAddressRequest {
        namespace,
        name,
        system,
    }): web::Path<ModuleAddressRequest>,
) -> Result<impl IntoResponse> {
    // should return a 302 to the download URL of the latest version available
    Ok(Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .finish())
}
