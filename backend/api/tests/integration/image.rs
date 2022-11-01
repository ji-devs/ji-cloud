mod recent;
mod tag;
mod user;

use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::{image::ImageId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
    service::Service,
};

async fn create(
    styles: &[Uuid],
    age_ranges: &[Uuid],
    affiliations: &[Uuid],
    categories: &[Uuid],
    tags: &[i16],
    port: u16,
) -> anyhow::Result<()> {
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
            "size": "Canvas",
        }))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<ImageId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::MetaKinds")
)]
async fn create_no_meta(port: u16) -> anyhow::Result<()> {
    create(&[], &[], &[], &[], &[], port).await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::MetaKinds")
)]
async fn create_with_styles(port: u16) -> anyhow::Result<()> {
    create(
        &[
            "6389eaa0-de76-11ea-b7ab-0399bcf84df2".parse()?,
            "6389ff7c-de76-11ea-b7ab-9b5661dd4f70".parse()?,
        ],
        &[],
        &[],
        &[],
        &[],
        port,
    )
    .await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::MetaKinds")
)]
async fn create_with_meta(port: u16) -> anyhow::Result<()> {
    create(
        &["6389eaa0-de76-11ea-b7ab-0399bcf84df2".parse()?],
        &["f3722790-de76-11ea-b7ab-77b45e9af3ef".parse()?],
        &["c0cd4446-de76-11ea-b7ab-93987e8aa112".parse()?],
        &[],
        &[1],
        port,
    )
    .await
}

async fn create_error(kind: &str, id: &str, port: u16) -> anyhow::Result<()> {
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
            "size": "Canvas",
            kind: [id],
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

async fn create_error_tag(kind: &str, id: &i16, port: u16) -> anyhow::Result<()> {
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
            "size": "Canvas",
            kind: [id],
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_with_style_error(port: u16) -> anyhow::Result<()> {
    create_error("styles", "6389eaa0-de76-11ea-b7ab-0399bcf84df2", port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_with_affiliation_error(port: u16) -> anyhow::Result<()> {
    create_error("affiliations", "6389eaa0-de76-11ea-b7ab-0399bcf84df2", port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_with_age_range_error(port: u16) -> anyhow::Result<()> {
    create_error("age_ranges", "6389eaa0-de76-11ea-b7ab-0399bcf84df2", port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_with_category_error(port: u16) -> anyhow::Result<()> {
    create_error("categories", "6389eaa0-de76-11ea-b7ab-0399bcf84df2", port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_with_tags_error(port: u16) -> anyhow::Result<()> {
    create_error_tag("tags", &22, port).await
}

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::Image",
        "Fixture::MetaKinds",
        "Fixture::MetaImage"
    )
)]
async fn get_metadata(port: u16) -> anyhow::Result<()> {
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

async fn update(req: &serde_json::Value, port: u16) -> anyhow::Result<()> {
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

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::MetaKinds", "Fixture::Image")
)]
async fn update_empty(port: u16) -> anyhow::Result<()> {
    update(&json!({}), port).await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::MetaKinds", "Fixture::Image")
)]
async fn update_is_premium(port: u16) -> anyhow::Result<()> {
    update(&json!({"is_premium": true}), port).await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::MetaKinds", "Fixture::Image")
)]
async fn update_styles(port: u16) -> anyhow::Result<()> {
    update(&json!({"styles": ["6389eaa0-de76-11ea-b7ab-0399bcf84df2", "6389ff7c-de76-11ea-b7ab-9b5661dd4f70"]}), port
    )
    .await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::MetaKinds", "Fixture::Image")
)]
async fn update_tags(port: u16) -> anyhow::Result<()> {
    update(&json!({"tags": [0, 2]}), port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn browse(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    // create a new image resource
    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/image/browse", port))
        .query(&[("page", "0"), ("size", "Canvas")])
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".metadata.updated_at" => "[timestamp]"});

    Ok(())
}

// https://cloud.google.com/storage/docs/performing-resumable-uploads#single-chunk-upload
#[ignore]
#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image"),
    services("Service::GoogleCloudStorage")
)]
async fn upload_with_url(port: u16) -> anyhow::Result<()> {
    let file: Vec<u8> = include_bytes!("../../fixtures/images/ji-logo.png").to_vec();

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
#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn create_media_and_upload_with_url(port: u16) -> anyhow::Result<()> {
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
            "size": "Canvas",
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
