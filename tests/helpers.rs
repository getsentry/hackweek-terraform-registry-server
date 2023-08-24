use poem::{middleware::AddDataEndpoint, Route};
use terraform_registry_server::{build_app, configuration::Settings};

pub type TestClient = poem::test::TestClient<AddDataEndpoint<Route, Settings>>;

pub fn get_client() -> TestClient {
    let app = build_app(&Settings::default());
    TestClient::new(app)
}

pub async fn assert_handler_returns_json(uri: &str) {
    let client = get_client();
    let resp = client.get(uri).send().await;

    resp.assert_status_is_ok();
    resp.assert_content_type("application/json; charset=utf-8");
    let _ = resp.json().await;
}
