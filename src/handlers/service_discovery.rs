use actix_web::{get, web, Responder, Result};
use serde_json::Value;

#[get("/.well-known/terraform.json")]
pub async fn service_discovery() -> Result<impl Responder> {
    let services = r#"
        {
            "modules.v1": "/terraform/modules/v1"
        }
    "#;
    let response: Value = serde_json::from_str(services)?;
    Ok(web::Json(response))
}
