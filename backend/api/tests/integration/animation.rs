use http::StatusCode;
use serde_json::json;
use shared::domain::{animation::AnimationId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn create(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/animation", port))
        .login()
        .json(&json!({
            "name": "test",
            "description": "testest",
            "is_premium": false,
            "publish_at": (),
            "styles": [],
            "kind": "Gif",
            "is_looping": false,
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<AnimationId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[sqlx::test]
async fn get_metadata(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::Animation,
            Fixture::MetaKinds,
            Fixture::MetaAnimation,
        ],
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
            "http://0.0.0.0:{}/v1/animation/3de4bcb6-7d23-11eb-8c8a-fbaee84afb0a",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".metadata.updated_at" => "[timestamp]"});

    Ok(())
}

// todo: upload; s3
// todo: delete; s3
// todo: delete: edge case (never uploaded, should work even without s3)
// todo: update (no route)
