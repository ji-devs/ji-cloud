use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};
use http::StatusCode;
use macros::test_service;
use shared::domain::jig::codes::{
    instance::PlayerSessionInstanceResponse, JigCodeListResponse, JigCodeResponse,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn create_and_list(port: u16) -> anyhow::Result<()> {
    let name = "create";

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/codes", port))
        .json(&serde_json::json!({
            "jigId": "3a71522a-cd77-11eb-8dc1-af3e35f7c743",
            "settings": {
                "direction": "rtl",
                "scoring": false,
                "drag_assist": false,
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: JigCodeResponse = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1",name), body, { ".**.index" => "[index]", ".**.created_at" => "[timestamp]", ".**.expires_at" => "[timestamp]" });

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/codes/{}",
            port,
            body.index.to_string()
        ))
        .json(&serde_json::json!({
            "name": "test-name"
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let _resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/codes", port))
        .json(&serde_json::json!({
            "jigId": "3a71522a-cd77-11eb-8dc1-af3e35f7c743",
            "settings": {
                "direction": "rtl",
                "scoring": true,
                "drag_assist": false,
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/codes", port))
        .json(&serde_json::json!({}))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigCodeListResponse = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2",name), body, { ".**.index" => "[index]", ".**.created_at" => "[timestamp]", ".**.expires_at" => "[timestamp]" });

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn session_instance_play_count_flow(port: u16) -> anyhow::Result<()> {
    let name = "session_instance_play_count_flow";

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/codes/instance", port))
        .json(&serde_json::json!({
            "code": 123456,
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: PlayerSessionInstanceResponse = resp.json().await?;

    let token = body.token.clone();

    insta::assert_json_snapshot!(format!("{}-1",name), body, {".**.token" => "[instance_token]"});

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/codes/instance/complete",
            port
        ))
        .json(&serde_json::json!({
            "token": token,
            "session": {
                "modules": [
                    {
                        "Matching": {
                            "stable_module_id": "00000000-0000-0000-0000-000000000000",
                            "rounds": [
                                {
                                    "2": {
                                        "failed_tries": 3
                                    },
                                    "0": {
                                        "failed_tries": 0
                                    },
                                    "3": {
                                        "failed_tries": 1
                                    }
                                },
                                {
                                    "1": {
                                        "failed_tries": 0
                                    },
                                    "2": {
                                        "failed_tries": 0
                                    },
                                    "0": {
                                        "failed_tries": 1
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    Ok(())
}
