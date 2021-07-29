use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::image::tag::{ImageTagCreateRequest, ImageTagUpdateRequest};

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn create_conflict() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn list() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

async fn update(index: i16, req: ImageTagUpdateRequest) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn update_no_index() -> anyhow::Result<()> {
    update(
        0,
        ImageTagUpdateRequest {
            display_name: Some("test".to_owned()),
            index: None,
        },
    )
    .await
}

#[actix_rt::test]
async fn update_with_index() -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: Some("test".to_owned()),
            index: Some(15),
        },
    )
    .await
}

#[actix_rt::test]
async fn update_none() -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: None,
            index: None,
        },
    )
    .await
}

#[actix_rt::test]
async fn update_only_index() -> anyhow::Result<()> {
    update(
        1,
        ImageTagUpdateRequest {
            display_name: None,
            index: Some(3),
        },
    )
    .await
}

#[actix_rt::test]
async fn update_conflict() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/image/tag/{}", port, 1))
        .json(&ImageTagUpdateRequest {
            display_name: None,
            index: Some(0),
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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image, Fixture::MetaKinds], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!("http://0.0.0.0:{}/v1/image/tag/2", port,))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    app.stop(false).await;

    Ok(())
}
