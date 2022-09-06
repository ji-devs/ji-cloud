use http::StatusCode;
use sqlx::PgPool;

use crate::{fixture::Fixture, helpers::initialize_server};

#[sqlx::test]
async fn create_401_no_auth(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[sqlx::test]
async fn create_basic(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn create_basic_bad_password(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth("test@test.test", Some("wrongPAssword"))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}
