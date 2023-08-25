use poem::{handler, web::Json};
use serde_json::Value;

#[handler]
pub async fn service_discovery() -> Json<Value> {
    Json(serde_json::json!({
        "modules.v1": "/v1/modules/",
        "providers.v1": "/v1/modules/"
    }))
}
