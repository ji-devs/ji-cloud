use http::StatusCode;
use shared::error::{ApiError, EmptyError};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

async fn unauthorized(route: &str) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserNoPerms]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/{}", port, route))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let body: ApiError<EmptyError> = resp.json().await?;

    assert_eq!(body.code, StatusCode::UNAUTHORIZED);

    Ok(())
}

#[actix_rt::test]
async fn get_image() -> anyhow::Result<()> {
    unauthorized("v1/image/00000000-0000-0000-0000-000000000000").await
}

#[actix_rt::test]
async fn get_jig() -> anyhow::Result<()> {
    unauthorized("v1/jig/00000000-0000-0000-0000-000000000000").await
}

#[actix_rt::test]
async fn get_module() -> anyhow::Result<()> {
    unauthorized("v1/module/00000000-0000-0000-0000-000000000000").await
}

#[actix_rt::test]
async fn get_animation() -> anyhow::Result<()> {
    unauthorized("v1/animation/00000000-0000-0000-0000-000000000000").await
}

#[actix_rt::test]
async fn get_category() -> anyhow::Result<()> {
    unauthorized("v1/category?ids=00000000-0000-0000-0000-000000000000").await
}
