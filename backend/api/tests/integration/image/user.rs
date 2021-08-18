use http::StatusCode;
use serde_json::json;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

async fn list(query: &[(&str, &str)]) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/image", port))
        .query(query)
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
async fn list_kind() -> anyhow::Result<()> {
    list(&[("kind", "Sticker")]).await
}

#[actix_rt::test]
async fn list_all() -> anyhow::Result<()> {
    list(&[]).await
}

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/image", port))
        .json(&json!({
            "kind": "Sticker",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".**.id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/me/image/{}",
            port, "89125d88-ffaa-11eb-86a5-9fd50ab8d8df"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".**.id" => "[id]"});

    Ok(())
}

// needs s3
#[ignore]
#[actix_rt::test]
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/user/me/image/{}",
            port, "89125d88-ffaa-11eb-86a5-9fd50ab8d8df"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".**.id" => "[id]"});

    Ok(())
}
