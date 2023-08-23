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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test, App,
    };

    #[actix_web::test]
    async fn test_service_discovery() {
        let app = test::init_service(App::new().service(service_discovery)).await;

        let req = test::TestRequest::get()
            .insert_header(ContentType::json())
            .uri("/.well-known/terraform.json")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
