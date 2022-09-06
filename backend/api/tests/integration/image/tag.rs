use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::image::tag::{ImageTagCreateRequest, ImageTagUpdateRequest};
use shared::domain::meta::ImageTagIndex;
use sqlx::PgPool;

#[sqlx::test]
async fn create(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/image/tag/{}", port, 3))
        .json(&ImageTagCreateRequest {
            display_name: "test name".to_owned(),
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".id" => "[id]" });

    Ok(())
}

#[sqlx::test]
async fn create_conflict(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/image/tag/{}", port, 0))
        .json(&ImageTagCreateRequest {
            display_name: "test name".to_owned(),
        })
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::CONFLICT);

    Ok(())
}

#[sqlx::test]
async fn list(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    log::info!("making request");

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/image/tag/all", port,))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".**.id" => "[id]" });

    Ok(())
}

async fn update(index: i16, req: ImageTagUpdateRequest, pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/image/tag/{}", port, index,))
        .json(&req)
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/image/tag/all", port,))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".**.id" => "[id]" });

    Ok(())
}

#[sqlx::test]
async fn update_no_index(pool: PgPool) -> anyhow::Result<()> {
    update(
        0,
        ImageTagUpdateRequest {
            display_name: Some("test".to_owned()),
            index: None,
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_with_index(pool: PgPool) -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: Some("test".to_owned()),
            index: Some(ImageTagIndex(15)),
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_none(pool: PgPool) -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: None,
            index: None,
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_only_index(pool: PgPool) -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: None,
            index: Some(ImageTagIndex(3)),
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_conflict(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/image/tag/{}", port, 1))
        .json(&ImageTagUpdateRequest {
            display_name: None,
            index: Some(ImageTagIndex(0)),
        })
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::CONFLICT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/image/tag/all", port,))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, { ".**.id" => "[id]" });

    Ok(())
}

#[sqlx::test]
async fn delete(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::Image, Fixture::MetaKinds],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!("http://0.0.0.0:{}/v1/image/tag/2", port,))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    Ok(())
}
