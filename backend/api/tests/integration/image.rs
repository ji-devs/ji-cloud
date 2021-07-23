mod recent;
mod tag;

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
    tags: &[Uuid],
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

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
            "tags": tags,
            "kind": "Canvas",
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<ImageId> = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn create_no_meta() -> anyhow::Result<()> {
    create(&[], &[], &[], &[], &[]).await
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
        &["5e72c62e-a3a4-11eb-96e7-c78c34eb32ee".parse()?],
    )
    .await
}

async fn create_error(kind: &str, id: &str) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

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
            "tags": [],
            "kind": "Canvas",
            kind: [id],
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

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
async fn create_with_tags_error() -> anyhow::Result<()> {
    create_error("tags", "6389eaa0-de76-11ea-b7ab-0399bcf84df2").await
}

#[actix_rt::test]
async fn get_metadata() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Image,
            Fixture::MetaImage,
        ],
        &[],
    )
    .await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".metadata.updated_at" => "[timestamp]"});

    Ok(())
}

// todo: search; missing algolia, s3
// todo: upload; missing algolia, s3
// todo: delete; missing algolia, s3
// todo: delete: edge case (never uploaded, should work even without s3), missing algolia

async fn update(req: &serde_json::Value) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::MetaKinds, Fixture::Image], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

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

#[actix_rt::test]
async fn update_tags() -> anyhow::Result<()> {
    update(&json!({"tags": ["591a2a64-a3a4-11eb-96e7-6bc0e819bc5f", "5b032222-a3a4-11eb-96e7-dbc5742f1640"]})).await
}

// https://cloud.google.com/storage/docs/performing-resumable-uploads#single-chunk-upload
#[ignore]
#[actix_rt::test]
async fn upload_with_url() -> anyhow::Result<()> {
    let file: Vec<u8> = include_bytes!("../../fixtures/ji-logo.png").to_vec();

    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    // get an upload URL for the new media
    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/image/8cca720a-c4bb-11eb-8edf-63da1d86939c/raw",
            port,
        ))
        .json(&json!({ "file_size": &file.len() }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let resp: shared::domain::image::ImageUploadResponse = resp.json().await?;

    let resp = client
        .put(&resp.session_uri)
        .header(reqwest::header::CONTENT_LENGTH, &file.len().to_string())
        .body(file)
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);

    Ok(())
}

#[ignore]
#[actix_rt::test]
async fn create_media_and_upload_with_url() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    // create a new image resource
    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/image", port))
        .json(&json!({
            "name": "test_img",
            "description": "test_descip",
            "is_premium": false,
            "publish_at": (),
            "styles": [],
            "age_ranges": [],
            "affiliations": [],
            "tags": [],
            "categories": [],
            "kind": "Canvas",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(StatusCode::CREATED, resp.status());

    let resp: shared::domain::image::CreateResponse = resp.json().await?;

    let id = resp.id.0;

    // get an upload URL for the new media
    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/image/{}/raw",
            port,
            id.to_string()
        ))
        .json(&json!({"file_size": 5}))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let resp: shared::domain::image::ImageUploadResponse = resp.json().await?;

    let _url = resp.session_uri;

    // perform upload in single chunk
    let _resp = client;

    Ok(())
}
