use poem::test::TestClient;
use terraform_registry_server::{build_app, configuration};

#[tokio::test]
async fn service_discovery_works() {
    let settings = configuration::get_configuration().expect("failed to get test configuration");
    let app = build_app(&settings);
    let client = TestClient::new(app);

    let resp = client.get("/.well-known/terraform.json").send().await;

    resp.assert_status_is_ok();
}
