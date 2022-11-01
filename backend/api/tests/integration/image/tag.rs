use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};
use http::StatusCode;
use macros::test_service;
use shared::domain::image::tag::{ImageTagCreateRequest, ImageTagUpdateRequest};
use shared::domain::meta::ImageTagIndex;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn create(port: u16) -> anyhow::Result<()> {
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

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn create_conflict(port: u16) -> anyhow::Result<()> {
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

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn list(port: u16) -> anyhow::Result<()> {
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

async fn update(index: i16, req: ImageTagUpdateRequest, port: u16) -> anyhow::Result<()> {
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

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn update_no_index(port: u16) -> anyhow::Result<()> {
    update(
        0,
        ImageTagUpdateRequest {
            display_name: Some("test".to_owned()),
            index: None,
        },
        port,
    )
    .await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn update_with_index(port: u16) -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: Some("test".to_owned()),
            index: Some(ImageTagIndex(15)),
        },
        port,
    )
    .await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn update_none(port: u16) -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: None,
            index: None,
        },
        port,
    )
    .await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn update_only_index(port: u16) -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: None,
            index: Some(ImageTagIndex(3)),
        },
        port,
    )
    .await
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn update_conflict(port: u16) -> anyhow::Result<()> {
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

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User, Fixture::Image, Fixture::MetaKinds")
)]
async fn delete(port: u16) -> anyhow::Result<()> {
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
