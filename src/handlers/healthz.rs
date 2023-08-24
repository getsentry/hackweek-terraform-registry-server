use poem::{handler, http::StatusCode, IntoResponse, Response};

#[handler]
pub async fn healthz() -> impl IntoResponse {
    Response::builder().status(StatusCode::OK).finish()
}
