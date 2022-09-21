mod recent;
mod tag;
mod user;

use http::StatusCode;
use serde_json::json;
use shared::domain::{image::ImageId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
    service::Service,
};

async fn create(
    styles: &[Uuid],
    age_ranges: &[Uuid],
    affiliations: &[Uuid],
    categories: &[Uuid],
    tags: &[i16],
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn create_no_meta(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create(&[], &[], &[], &[], &[], pool_opts, conn_opts).await
}

#[sqlx::test]
async fn create_with_styles(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create(
        &[
            "6389eaa0-de76-11ea-b7ab-0399bcf84df2".parse()?,
            "6389ff7c-de76-11ea-b7ab-9b5661dd4f70".parse()?,
        ],
        &[],
        &[],
        &[],
        &[],
        pool_opts,
        conn_opts,
    )
    .await
}

#[sqlx::test]
async fn create_with_meta(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create(
        &["6389eaa0-de76-11ea-b7ab-0399bcf84df2".parse()?],
        &["f3722790-de76-11ea-b7ab-77b45e9af3ef".parse()?],
        &["c0cd4446-de76-11ea-b7ab-93987e8aa112".parse()?],
        &[],
        &[1],
        pool_opts,
        conn_opts,
    )
    .await
}

async fn create_error(
    kind: &str,
    id: &str,
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

async fn create_error_tag(
    kind: &str,
    id: &i16,
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn create_with_style_error(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create_error(
        "styles",
        "6389eaa0-de76-11ea-b7ab-0399bcf84df2",
        pool_opts,
        conn_opts,
    )
    .await
}

#[sqlx::test]
async fn create_with_affiliation_error(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create_error(
        "affiliations",
        "6389eaa0-de76-11ea-b7ab-0399bcf84df2",
        pool_opts,
        conn_opts,
    )
    .await
}

#[sqlx::test]
async fn create_with_age_range_error(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create_error(
        "age_ranges",
        "6389eaa0-de76-11ea-b7ab-0399bcf84df2",
        pool_opts,
        conn_opts,
    )
    .await
}

#[sqlx::test]
async fn create_with_category_error(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create_error(
        "categories",
        "6389eaa0-de76-11ea-b7ab-0399bcf84df2",
        pool_opts,
        conn_opts,
    )
    .await
}

#[sqlx::test]
async fn create_with_tags_error(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    create_error_tag("tags", &22, pool_opts, conn_opts).await
}

#[sqlx::test]
async fn get_metadata(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::MetaKinds,
            Fixture::Image,
            Fixture::MetaImage,
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

async fn update(
    req: &serde_json::Value,
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::MetaKinds, Fixture::Image],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn update_empty(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    update(&json!({}), pool_opts, conn_opts).await
}

#[sqlx::test]
async fn update_is_premium(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    update(&json!({"is_premium": true}), pool_opts, conn_opts).await
}

#[sqlx::test]
async fn update_styles(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    update(&json!({"styles": ["6389eaa0-de76-11ea-b7ab-0399bcf84df2", "6389ff7c-de76-11ea-b7ab-9b5661dd4f70"]}), pool_opts,
conn_opts
    )
    .await
}

#[sqlx::test]
async fn update_tags(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    update(&json!({"tags": [0, 2]}), pool_opts, conn_opts).await
}

#[sqlx::test]
async fn browse(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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
#[sqlx::test]
async fn upload_with_url(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let file: Vec<u8> = include_bytes!("../../fixtures/images/ji-logo.png").to_vec();

    let app = initialize_server(
        &[Fixture::User, Fixture::Image],
        &[Service::GoogleCloudStorage],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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
#[sqlx::test]
async fn create_media_and_upload_with_url(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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
