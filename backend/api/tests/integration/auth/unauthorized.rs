use http::StatusCode;
use macros::test_service;
use shared::error::{ApiError, EmptyError};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{fixture::Fixture, helpers::setup_service};

async fn unauthorized(route: &str, port: u16) -> anyhow::Result<()> {
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

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
#[ignore]
async fn get_image(port: u16) -> anyhow::Result<()> {
    unauthorized("v1/image/00000000-0000-0000-0000-000000000000", port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn get_animation(port: u16) -> anyhow::Result<()> {
    unauthorized("v1/animation/00000000-0000-0000-0000-000000000000", port).await
}
