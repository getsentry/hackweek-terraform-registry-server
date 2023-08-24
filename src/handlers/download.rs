#![allow(unused_variables)]
use crate::{configuration::Settings, handlers::ModuleAddressWithVersionRequest};
use poem::FromRequest;
use poem::{handler, http::StatusCode, web, IntoResponse, Request, Response, Result};
use std::path::Path;

#[handler]
pub async fn download(
    req: &Request,
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
        .join(&version);

    if !path.exists() {
        return Ok(Response::builder().status(StatusCode::NOT_FOUND).finish());
    }

    let static_file = poem::web::StaticFileRequest::from_request_without_body(req)
        .await
        .unwrap();

    let response = static_file.create_response(path, false)?;
    Ok(response.into_response())
}
