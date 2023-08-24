use crate::helpers::get_client;

#[tokio::test]
async fn healthz_works() {
    let client = get_client();

    let resp = client.get("/healthz").send().await;

    resp.assert_status_is_ok();
}
