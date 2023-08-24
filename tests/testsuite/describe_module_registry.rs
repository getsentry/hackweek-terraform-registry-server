use poem::http::StatusCode;

use crate::helpers::{assert_handler_returns_json, get_client};

mod versions {
    use super::*;

    #[tokio::test]
    async fn it_returns_json() {
        assert_handler_returns_json("/v1/modules/sentry/saas/project/versions").await;
    }

    #[tokio::test]
    async fn it_returns_404_on_invalid_module() {
        let client = get_client();
        let resp = client
            .get("/v1/modules/does/not/exist/versions")
            .send()
            .await;
        resp.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn it_returns_available_versions() {
        let client = get_client();
        let resp = client
            .get("/v1/modules/sentry/saas/project/versions")
            .send()
            .await;

        resp.assert_status_is_ok();

        let expected_json = serde_json::json!(
            {
                "modules": [
                    {
                        "versions": [
                            {"version": "1.0.0"},
                            {"version": "2.1.0"},
                            {"version": "3.2.1"},
                        ]
                    }
                ]
            }
        );

        resp.assert_json(expected_json).await;
    }
}
