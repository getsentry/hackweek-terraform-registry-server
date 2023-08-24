#![allow(unused_variables)]
use poem::{handler, web, IntoResponse};

use crate::{configuration::Settings, handlers::ModuleAddressWithVersionRequest};

#[handler]
pub async fn download(
    settings: web::Data<&Settings>,
    web::Path(ModuleAddressWithVersionRequest {
        namespace,
        name,
        system,
        version,
    }): web::Path<ModuleAddressWithVersionRequest>,
) -> impl IntoResponse {
    todo!()
}
