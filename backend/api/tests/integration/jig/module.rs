use http::StatusCode;

use shared::domain::jig::module::{
    body::memory, ModuleBody, ModuleCreateRequest, ModuleUpdateRequest,
};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create_default() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/module",
            port
        ))
        .json(&ModuleCreateRequest::default())
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {
        ".**.id" => "[id]",
        ".**.created_at" => "[created_at]",
        ".**.updated_at" => "[updated_at]"});

    let id = body.get("id").unwrap().as_str().unwrap();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/module/{}",
            port, id,
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {
        ".**.id" => "[id]",
        ".**.created_at" => "[created_at]",
        ".**.updated_at" => "[updated_at]"});

    Ok(())
}

#[actix_rt::test]
async fn update_empty() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/module/0cbfdd82-7c83-11eb-9f77-d7d86264c3bc",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/module/0cbfdd82-7c83-11eb-9f77-d7d86264c3bc",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    Ok(())
}

#[actix_rt::test]
async fn update_contents() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/module/0cc03a02-7c83-11eb-9f77-f77f9ad65e9a",
            port
        ))
        .login()
        .json(&ModuleUpdateRequest {
            body: Some(
                ModuleBody::MemoryGame(memory::ModuleData {
                    content: Some(memory::Content {
                        ..memory::Content::default()
                    })
                })
            ),
            is_complete: Some(true),

            ..ModuleUpdateRequest::default()
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/module/0cc03a02-7c83-11eb-9f77-f77f9ad65e9a",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    Ok(())
}
