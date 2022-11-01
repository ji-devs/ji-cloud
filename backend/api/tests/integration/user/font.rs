use http::StatusCode;
use macros::test_service;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::UserFonts")
)]
async fn get_all(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/font", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::UserFonts")
)]
async fn update(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/user/me/font/3", port))
        .json(&json!({ "name": "ArialBlackRegular" }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/font", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::UserFonts")
)]
async fn delete(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!("http://0.0.0.0:{}/v1/user/me/font/1", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/font", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::UserFonts")
)]
async fn create(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/font", port))
        .json(&json!({ "name": "ComputerModernRegular" }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/font", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
