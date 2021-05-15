use http::StatusCode;
use serde_json::json;
use shared::domain::{animation::AnimationId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/animation", port))
        .login()
        .json(&json!({
            "name": "test",
            "description": "testest",
            "is_premium": false,
            "publish_at": (),
            "variant": "Gif",
            "is_looping": false,
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<AnimationId> = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn get_metadata() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::MetaKinds, Fixture::Animation]).await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".metadata.updated_at" => "[timestamp]"});

    Ok(())
}

// todo: upload; s3
// todo: delete; s3
// todo: delete: edge case (never uploaded, should work even without s3)
// todo: update (no route)
