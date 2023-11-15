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
                "display_score": false,
                "track_assessments": false,
                "drag_assist": false,
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: JigCodeResponse = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1",name), body, { ".**.index" => "[index]", ".**.expires_at" => "[timestamp]" });

    let _resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/codes", port))
        .json(&serde_json::json!({
            "jigId": "3a71522a-cd77-11eb-8dc1-af3e35f7c743",
            "settings": {
                "direction": "rtl",
                "display_score": true,
                "track_assessments": false,
                "drag_assist": false,
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/codes", port))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigCodeListResponse = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2",name), body, { ".**.index" => "[index]", ".**.expires_at" => "[timestamp]"  });

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn session_instance_play_count_flow(port: u16) -> anyhow::Result<()> {
    let name = "session_instance_play_count_flow";

    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .user_agent("mocked user agent")
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/codes/instance", port))
        .json(&serde_json::json!({
            "code": 1234,
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
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);








    // let resp = client
    //     .get(&format!(
    //         "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/play-count",
    //         port,
    //     ))
    //     .login()
    //     .send()
    //     .await?
    //     .error_for_status()?;

    // let body: serde_json::Value = resp.json().await?;

    // insta::assert_json_snapshot!(format!("{}-2", name), body);

    Ok(())
}
