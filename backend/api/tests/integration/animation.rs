use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::{animation::AnimationId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create(port: u16) -> anyhow::Result<()> {
    let name = "create";

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

    insta::assert_json_snapshot!(format!("{}",name), body, {".id" => "[id]"});

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::Animation",
        "Fixture::MetaKinds",
        "Fixture::MetaAnimation"
    )
)]
async fn get_metadata(port: u16) -> anyhow::Result<()> {
    let name = "get_metadata";

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

    insta::assert_json_snapshot!(format!("{}",name), body, {".metadata.updated_at" => "[timestamp]"});

    Ok(())
}

// todo: upload; s3
// todo: delete; s3
// todo: delete: edge case (never uploaded, should work even without s3)
// todo: update (no route)
