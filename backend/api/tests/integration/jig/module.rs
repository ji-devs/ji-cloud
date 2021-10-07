use http::StatusCode;

use shared::domain::jig::module::{
    body::memory, ModuleBody, ModuleCreateRequest, ModuleGetRequest, ModuleId, ModuleUpdateRequest,
    StableModuleId, StableOrUniqueId,
};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn get_live() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/live/module",
            port
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Unique(ModuleId(uuid::Uuid::parse_str(
                "a6b24970-1dd7-11ec-8426-57136b411853",
            )?)),
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/live/module",
            port
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(
                "0cc03a02-7c83-11eb-9f77-f77f9ad65e9a",
            )?)),
        })
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
async fn create_default() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
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

    let id = body
        .get("id")
        .expect("has id")
        .as_str()
        .expect("convert to str");

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port,
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(id)?)),
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {
        ".**.id" => "[id]",
        ".**.stable_id" => "[stable_id]",
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
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port
        ))
        .json(&ModuleUpdateRequest {
            id: StableOrUniqueId::Unique(ModuleId(uuid::Uuid::parse_str(
                "a6b24a42-1dd7-11ec-8426-a7165f9281a2",
            )?)),
            is_complete: None,
            body: None,
            index: None,
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Unique(ModuleId(uuid::Uuid::parse_str(
                "a6b24a42-1dd7-11ec-8426-a7165f9281a2",
            )?)),
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(
                "0cc03a02-7c83-11eb-9f77-f77f9ad65e9a",
            )?)),
        })
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
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port
        ))
        .login()
        .json(&ModuleUpdateRequest {
            id: StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(
                "0cc03a02-7c83-11eb-9f77-f77f9ad65e9a",
            )?)),
            body: Some(ModuleBody::MemoryGame(memory::ModuleData {
                content: Some(memory::Content {
                    ..memory::Content::default()
                }),
            })),
            is_complete: Some(true),
            index: None,
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(
                "0cc03a02-7c83-11eb-9f77-f77f9ad65e9a",
            )?)),
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft/module",
            port
        ))
        .json(&ModuleGetRequest {
            id: StableOrUniqueId::Unique(ModuleId(uuid::Uuid::parse_str(
                "a6b24a42-1dd7-11ec-8426-a7165f9281a2",
            )?)),
        })
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
