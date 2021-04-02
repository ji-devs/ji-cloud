use http::StatusCode;
use serde_json::json;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn update_empty() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Modules]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/module/5c8b6458-8840-11eb-8cf7-9bf404dc7386",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/module/5c8b6458-8840-11eb-8cf7-9bf404dc7386",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    Ok(())
}

#[actix_rt::test]
async fn update_contents() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Modules]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/module/5c8b6458-8840-11eb-8cf7-9bf404dc7386",
            port
        ))
        .login()
        .json(&json!({"body": {}}))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/module/5c8b6458-8840-11eb-8cf7-9bf404dc7386",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    Ok(())
}
