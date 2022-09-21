use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::jig::player::{
    instance::PlayerSessionInstanceResponse, JigPlayerSession, JigPlayerSessionListResponse,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test]
async fn list(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/player",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigPlayerSessionListResponse = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".**.expires_at" => "[timestamp]" });

    Ok(())
}

#[sqlx::test]
async fn create(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/player", port))
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

    let body: JigPlayerSession = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".**.index" => "[index]", ".**.expires_at" => "[timestamp]" });

    let _resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/player", port))
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
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/3a71522a-cd77-11eb-8dc1-af3e35f7c743/player",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigPlayerSessionListResponse = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".**.index" => "[index]", ".**.expires_at" => "[timestamp]"  });

    Ok(())
}

#[sqlx::test]
async fn session_instance_play_count_flow(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .user_agent("mocked user agent")
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig/player/instance", port))
        .json(&serde_json::json!({
            "index": 1234,
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: PlayerSessionInstanceResponse = resp.json().await?;

    let token = body.token.clone();

    insta::assert_json_snapshot!(body, {".**.token" => "[instance_token]"});

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/player/instance/complete",
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

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/play-count",
            port,
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
