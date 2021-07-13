use http::StatusCode;

use crate::{fixture::Fixture, helpers::initialize_server};

#[actix_rt::test]
async fn create_401_no_auth() -> anyhow::Result<()> {
    let app = initialize_server(&[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn create_basic() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

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

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn create_basic_bad_password() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/session", port))
        .basic_auth("test@test.test", Some("wrongPAssword"))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    app.stop(false).await;

    Ok(())
}
