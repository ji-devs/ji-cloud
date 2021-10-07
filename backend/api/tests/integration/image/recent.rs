use http::StatusCode;
use serde_json::json;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use shared::domain::image::recent::UserRecentImageListRequest;

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".last_used" => "[timestamp]"});

    Ok(())
}

#[actix_rt::test]
async fn create_conflict() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".last_used" => "[timestamp]"});

    Ok(())
}

#[actix_rt::test]
async fn list_no_limit() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn list_with_limit() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn update() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

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

    insta::assert_json_snapshot!(body, {".**.last_used" => "[timestamp]"});

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/recent/image", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body_2: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body_2, {".**.last_used" => "[timestamp]"});

    Ok(())
}

#[actix_rt::test]
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    insta::assert_json_snapshot!(body); //, {".**.last_used" => "[timestamp]"});

    Ok(())
}
