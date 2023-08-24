use poem::test::TestClient;
use terraform_registry_server::{build_app, configuration::Settings};

#[tokio::test]
async fn healthz_works() {
    let app = build_app(&Settings::default());
    let client = TestClient::new(app);

    let resp = client.get("/healthz").send().await;

    resp.assert_status_is_ok();
}
