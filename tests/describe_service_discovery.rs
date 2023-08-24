use poem::test::TestClient;
use terraform_registry_server::{build_app, configuration::Settings};

#[tokio::test]
async fn it_returns_json() {
    let app = build_app(&Settings::default());
    let client = TestClient::new(app);

    let resp = client.get("/.well-known/terraform.json").send().await;

    resp.assert_status_is_ok();
    resp.assert_content_type("application/json; charset=utf-8");

    let expected_json = serde_json::json!(
        {
            "modules.v1": "/terraform/modules/v1"
        }
    );

    resp.assert_json(expected_json).await;
}
