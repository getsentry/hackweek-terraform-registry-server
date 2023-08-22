use actix_web::{get, HttpResponse, Responder};

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}
