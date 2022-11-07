use http::StatusCode;
use macros::test_service;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};
use shared::domain::image::recent::UserRecentImageListRequest;

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn create(port: u16) -> anyhow::Result<()> {
    let name = "create";
    let client = reqwest::Client::new();

    let resp = client
        .put(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .json(&json!({
            "id": "3095d05e-f2c7-11ea-89c3-3b621dd74a1f",
            "library": "User",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}",name), body, {".last_used" => "[timestamp]"});

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn create_conflict(port: u16) -> anyhow::Result<()> {
    let name = "create_conflict";
    let client = reqwest::Client::new();

    let resp = client
        .put(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .json(&json!({
            //"id": "3095d05e-f2c7-11ea-89c3-3b621dd74a1f",
            "id": "8cca6f3a-c4bb-11eb-8edf-13c75672da8f",
            "library": "User",
        }))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}",name),body , {".last_used" => "[timestamp]"});

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn list_no_limit(port: u16) -> anyhow::Result<()> {
    let name = "list_no_limit";
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn list_with_limit(port: u16) -> anyhow::Result<()> {
    let name = "list_with_limit";
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .login()
        .query(&UserRecentImageListRequest { limit: 3 })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn update(port: u16) -> anyhow::Result<()> {
    let name = "update";
    let client = reqwest::Client::new();

    // updating
    let resp = client
        .put(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .json(&json!({
            "id": "8cca719c-c4bb-11eb-8edf-f7accb638a15",
            "library": "User",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1",name), body, {".**.last_used" => "[timestamp]"});

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body_2: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2",name), body_2, {".**.last_used" => "[timestamp]"});

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn delete(port: u16) -> anyhow::Result<()> {
    let name = "delete";
    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/user/me/recent/image/{}",
            port, "8cca719c-c4bb-11eb-8edf-f7accb638a15"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body); //, {".**.last_used" => "[timestamp]"});

    Ok(())
}
