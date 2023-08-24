use poem::http::StatusCode;

use crate::helpers::{assert_handler_returns_json, get_client};

mod versions {
    use crate::helpers;

    use super::*;

    #[tokio::test]
    async fn it_returns_json() {
        assert_handler_returns_json("/v1/modules/namespace-a/module-x/system-2/versions").await;
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
            .get("/v1/modules/namespace-a/module-x/system-2/versions")
            .send()
            .await;

        resp.assert_status_is_ok();

        let expected_json = serde_json::json!(
            {
                "modules": [
                    {
                        "versions": [
                            {"version": "2.1.21.tar.xz"},
                            {"version": "4.1.11.tar.xz"},
                            {"version": "2.0.11.tar.xz"},
                            {"version": "2.0.21.tar.xz"},
                            {"version": "2.0.12.tar.xz"},
                            {"version": "2.0.22.tar.xz"},
                        ]
                    }
                ]
            }
        );

        let actual_json = resp.json().await;

        helpers::assert_json_eq(expected_json, actual_json);
    }
}

mod downloads {}
