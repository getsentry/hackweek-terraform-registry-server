use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::handlers::ModuleAddressWithVersionRequest;
use crate::state::AppState;

#[get("/{namespace}/{name}/{system}/{version}")]
pub async fn download(
    _req: HttpRequest,
    state: web::Data<AppState>,
    module: web::Path<ModuleAddressWithVersionRequest>,
) -> HttpResponse {
    todo!()
}
