use http::StatusCode;
use macros::test_service;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{fixture::Fixture, helpers::setup_service};

#[test_service(setup = "setup_service", fixtures(""))]
async fn create_401_no_auth(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_basic(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth("test@test.test", Some("password1"))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;
    body.as_object()
        .expect("body wasn't a object")
        .contains_key("csrf");

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_basic_bad_password(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth("test@test.test", Some("wrongPAssword"))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}
