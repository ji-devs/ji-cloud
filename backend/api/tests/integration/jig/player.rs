use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::jig::player::{JigPlayerSession, JigPlayerSessionCode, JigPlayerSessionToken};

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/player/1234", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigPlayerSession = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/player", port))
        .json(&serde_json::json!({
            "jig_id": "3a71522a-cd77-11eb-8dc1-af3e35f7c743",
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

    let body: JigPlayerSessionCode = resp.json().await?;

    let idx = body.index;

    insta::assert_json_snapshot!(body);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/player",
            port
        ))
        .login()
        .send()
        .await?;
    //.error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigPlayerSessionCode = resp.json().await?;

    assert_eq!(idx, body.index);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/player/{}",
            port, body.index
        ))
        .login()
        .send()
        .await?;
    //.error_for_status()?;

    app.stop(false).await;

    let body: JigPlayerSession = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn create_player_session() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .user_agent("USER_AGENT")
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/player/instance", port))
        .json(&serde_json::json!({
            "session_index": 1234,
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: JigPlayerSessionToken = resp.json().await?;

    let token = body.token;

    println!("{}", token);

    let token_resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/player/instance/complete",
            port
        ))
        .json(&serde_json::json!({
            "jig_id": "0cc084bc-7c83-11eb-9f77-e3218dffb008",
            "token": token,
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(token_resp.status(), StatusCode::NO_CONTENT);

    Ok(())
}
