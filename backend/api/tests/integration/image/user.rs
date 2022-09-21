use http::StatusCode;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

async fn list(
    query: &[(&str, &str)],
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn list_kind(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    list(&[("kind", "Sticker")], pool_opts, conn_opts).await
}

#[sqlx::test]
async fn list_all(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    list(&[], pool_opts, conn_opts).await
}

#[sqlx::test]
async fn create(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/image", port))
        .json(&json!({
            "size": "Sticker",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.id" => "[id]"});

    Ok(())
}

#[sqlx::test]
async fn get(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body, {".**.id" => "[id]"});

    Ok(())
}

// needs s3
#[ignore]
#[sqlx::test]
async fn delete(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Image], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    insta::assert_json_snapshot!(body, {".**.id" => "[id]"});

    Ok(())
}
