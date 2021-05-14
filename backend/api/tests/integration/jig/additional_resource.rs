use http::StatusCode;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use shared::domain::jig::additional_resource::{
    AdditionalResourceCreateRequest, AdditionalResourceUpdateRequest,
};
use url::Url;

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/additional-resource",
            port
        ))
        .login()
        .json(&AdditionalResourceCreateRequest {
            url: "http://test.tset.test/tes".to_string(),
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/additional-resource/41b8d0b4-aaff-4942-88ba-1a32fecdbd23",
            port
        ))
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
async fn update() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/additional-resource/41b8d0b4-aaff-4942-88ba-1a32fecdbd23",
            port
        ))
        .login()
        .json(&AdditionalResourceCreateRequest{ url: "url://updated.s.s".to_string() })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/additional-resource/41b8d0b4-aaff-4942-88ba-1a32fecdbd23",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[actix_rt::test]
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/additional-resource/41b8d0b4-aaff-4942-88ba-1a32fecdbd23",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/additional-resource/41b8d0b4-aaff-4942-88ba-1a32fecdbd23",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    app.stop(false).await;

    Ok(())
}
