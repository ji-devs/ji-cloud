use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::jig::additional_resource::AdditionalResourceCreateRequest;

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/additional-resource",
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
async fn get_draft() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/additional-resource/286b8390-1dd9-11ec-8426-fbeb80c504d9",
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
async fn get_live() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/live/additional-resource/286b828c-1dd9-11ec-8426-571b03b2d3df",
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
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/additional-resource/286b828c-1dd9-11ec-8426-571b03b2d3df",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/additional-resource/286b828c-1dd9-11ec-8426-571b03b2d3df",
            port
        ))
        .login()
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    app.stop(false).await;

    Ok(())
}
