use crate::helpers::{assert_handler_returns_json, get_client};

#[tokio::test]
async fn it_returns_json() {
    assert_handler_returns_json("/.well-known/terraform.json").await;
}

#[tokio::test]
async fn it_answers_terraform_service_discovery_request() {
    let client = get_client();

    let resp = client.get("/.well-known/terraform.json").send().await;

    resp.assert_status_is_ok();

    let expected_json = serde_json::json!(
        {
            "modules.v1": "/terraform/modules/v1"
        }
    );

    resp.assert_json(expected_json).await;
}
