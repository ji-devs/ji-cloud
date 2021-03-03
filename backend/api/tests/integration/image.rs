use http::StatusCode;
use serde_json::json;
use shared::domain::{image::ImageId, CreateResponse};
use uuid::Uuid;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

async fn create(
    styles: &[Uuid],
    age_ranges: &[Uuid],
    affiliations: &[Uuid],
    categories: &[Uuid],
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::MetaKinds]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/image", port))
        .login()
        .json(&json!({
            "name": "test",
            "description": "testest",
            "is_premium": false,
            "publish_at": (),
            "styles": styles,
            "age_ranges": age_ranges,
            "affiliations": affiliations,
            "categories": categories,
            "kind": "Canvas",
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<ImageId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn create_no_meta() -> anyhow::Result<()> {
    create(&[], &[], &[], &[]).await
}

#[actix_rt::test]
async fn create_with_styles() -> anyhow::Result<()> {
    create(
        &[
            "6389eaa0-de76-11ea-b7ab-0399bcf84df2".parse()?,
            "6389ff7c-de76-11ea-b7ab-9b5661dd4f70".parse()?,
        ],
        &[],
        &[],
        &[],
    )
    .await
}

#[actix_rt::test]
async fn create_with_meta() -> anyhow::Result<()> {
    create(
        &["6389eaa0-de76-11ea-b7ab-0399bcf84df2".parse()?],
        &["f3722790-de76-11ea-b7ab-77b45e9af3ef".parse()?],
        &["c0cd4446-de76-11ea-b7ab-93987e8aa112".parse()?],
        &[],
    )
    .await
}

async fn create_error(kind: &str, id: &str) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/image", port))
        .login()
        .json(&json!({
            "name": "test",
            "description": "testest",
            "is_premium": false,
            "publish_at": null,
            "styles": [],
            "age_ranges": [],
            "affiliations": [],
            "categories": [],
            "kind": "Canvas",
            kind: [id],
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn create_with_style_error() -> anyhow::Result<()> {
    create_error("styles", "6389eaa0-de76-11ea-b7ab-0399bcf84df2").await
}

#[actix_rt::test]
async fn create_with_affiliation_error() -> anyhow::Result<()> {
    create_error("affiliations", "6389eaa0-de76-11ea-b7ab-0399bcf84df2").await
}

#[actix_rt::test]
async fn create_with_age_range_error() -> anyhow::Result<()> {
    create_error("age_ranges", "6389eaa0-de76-11ea-b7ab-0399bcf84df2").await
}

#[actix_rt::test]
async fn create_with_category_error() -> anyhow::Result<()> {
    create_error("categories", "6389eaa0-de76-11ea-b7ab-0399bcf84df2").await
}

#[actix_rt::test]
async fn get_metadata() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::MetaKinds, Fixture::Image]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f",
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

// todo: search; missing algolia, s3
// todo: upload; missing algolia, s3
// todo: delete; missing algolia, s3
// todo: delete: edge case (never uploaded, should work even without s3), missing algolia

async fn update(req: &serde_json::Value) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::MetaKinds, Fixture::Image]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f",
            port
        ))
        .json(req)
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/image/3095d05e-f2c7-11ea-89c3-3b621dd74a1f",
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

#[actix_rt::test]
async fn update_empty() -> anyhow::Result<()> {
    update(&json!({})).await
}

#[actix_rt::test]
async fn update_is_premium() -> anyhow::Result<()> {
    update(&json!({"is_premium": true})).await
}

#[actix_rt::test]
async fn update_styles() -> anyhow::Result<()> {
    update(&json!({"styles": ["6389eaa0-de76-11ea-b7ab-0399bcf84df2", "6389ff7c-de76-11ea-b7ab-9b5661dd4f70"]})).await
}
