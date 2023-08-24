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
                            {"version": "2.1.21"},
                            {"version": "4.1.11"},
                            {"version": "2.0.11"},
                            {"version": "2.0.21"},
                            {"version": "2.0.12"},
                            {"version": "2.0.22"},
                        ]
                    }
                ]
            }
        );

        let actual_json = resp.json().await;

        helpers::assert_json_eq(expected_json, actual_json);
    }

    #[tokio::test]
    async fn it_returns_an_empty_list_with_no_versions() {
        let client = get_client();
        let resp = client
            .get("/v1/modules/namespace-d/module-x/system-2/versions")
            .send()
            .await;

        resp.assert_status_is_ok();

        let expected_json = serde_json::json!(
            {
                "modules": [
                    {
                        "versions": []
                    }
                ]
            }
        );

        let actual_json = resp.json().await;

        helpers::assert_json_eq(expected_json, actual_json);
    }

    #[tokio::test]
    async fn it_returns_x_terraform_get_header() {
        let client = get_client();
        let resp = client
            .get("/v1/modules/namespace-c/module-x/system-3/1.0.25/download")
            .send()
            .await;

        resp.assert_status(StatusCode::NO_CONTENT);
        resp.assert_header_exist("x-terraform-get");
    }

    #[tokio::test]
    async fn it_returns_valid_download_link_in_x_terraform_get_header() {
        use terraform_registry_server::configuration::Settings;
        let settings = Settings::default();

        let client = get_client();
        let resp = client
            .get("/v1/modules/namespace-c/module-x/system-3/1.0.25/download")
            .send()
            .await;

        resp.assert_status(StatusCode::NO_CONTENT);
        resp.assert_header_exist("x-terraform-get");

        resp.assert_header(
            "x-terraform-get",
            format!(
                "{}/download/namespace-c/module-x/system-3/1.0.25.tar.xz",
                settings.base_url,
            ),
        )
    }
}

mod downloads {
    use crate::helpers::get_client;

    #[tokio::test]
    async fn it_sets_valid_content_type_header() {
        let client = get_client();
        let resp = client
            .get("/download/namespace-c/module-x/system-3/1.0.25.tar.xz")
            .send()
            .await;

        resp.assert_status_is_ok();

        let content_type = resp.0.content_type();
        pretty_assertions::assert_eq!(Some("application/x-xz"), content_type);
    }

    // #[tokio::test]
    // async fn it_sets_valid_content_type_header() {
    //     let client = get_client();
    //     let resp = client
    //         .get("/download/namespace-c/module-x/system-3/1.0.25.tar.xz")
    //         .send()
    //         .await;

    //     resp.assert_status_is_ok();

    //     let content_type = resp.0.content_type();
    //     pretty_assertions::assert_eq!(Some("application/x-xz"), content_type);
    // }
}
