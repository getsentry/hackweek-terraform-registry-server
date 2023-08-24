use terraform_registry_server::{build_app, configuration::Settings};

#[tokio::test]
async fn it_returns_json() {
    let client = get_client();

    let resp = client.get("/.well-known/terraform.json").send().await;

    resp.assert_status_is_ok();
    resp.assert_content_type("application/json; charset=utf-8");
    let _ = resp.json().await;
}

#[tokio::test]
async fn it_answers_terraform_service_discovery_request() {
    let app = build_app(&Settings::default());
    let client = TestClient::new(app);

    let resp = client.get("/.well-known/terraform.json").send().await;

    resp.assert_status_is_ok();

    let expected_json = serde_json::json!(
        {
            "modules.v1": "/terraform/modules/v1"
        }
    );

    resp.assert_json(expected_json).await;
}
