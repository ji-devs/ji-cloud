use http::StatusCode;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn get_all(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::UserColors],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/color", port))
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
async fn update(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::UserColors],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/user/me/color/2", port))
        .json(&json!({
           "color": {
                "r": 210,
                "g": 124,
                "b": 19,
                "a": 255
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/color", port))
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
async fn delete(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::UserColors],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!("http://0.0.0.0:{}/v1/user/me/color/2", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/color", port))
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
async fn create(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::UserColors],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/color", port))
        .json(&json!({
           "color": {
                "r": 210,
                "g": 124,
                "b": 19,
                "a": 255
            }
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/color", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
