use http::StatusCode;
use serde_json::json;
use shared::domain::category::{
    CategoryTreeScope, CreateCategoryRequest, GetCategoryRequest, NewCategoryResponse,
};
use uuid::Uuid;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/category", port))
        .login()
        .json(&CreateCategoryRequest {
            name: "One".to_owned(),
            parent_id: None,
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let _body: NewCategoryResponse = resp.json().await?;

    Ok(())
}

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/category", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

async fn get_nested_categories(query: &GetCategoryRequest) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryNesting]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/category", port))
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

#[actix_rt::test]
async fn nested_top_level() -> anyhow::Result<()> {
    get_nested_categories(&GetCategoryRequest::default()).await
}

#[actix_rt::test]
async fn nested_whole_tree() -> anyhow::Result<()> {
    get_nested_categories(&GetCategoryRequest {
        scope: Some(CategoryTreeScope::Decendants),
        ids: vec![],
    })
    .await
}

#[actix_rt::test]
async fn nested_overlapping() -> anyhow::Result<()> {
    get_nested_categories(&GetCategoryRequest {
        scope: Some(CategoryTreeScope::Decendants),
        ids: vec![
            "afbce03c-e90f-11ea-8281-cfde02f6b582".parse()?,
            "e315d3b2-e90f-11ea-8281-73cd69c14821".parse()?,
        ],
    })
    .await
}

#[actix_rt::test]
async fn nested_ancestors() -> anyhow::Result<()> {
    get_nested_categories(&GetCategoryRequest {
        scope: Some(CategoryTreeScope::Ancestors),
        ids: vec!["e315d3b2-e90f-11ea-8281-73cd69c14821".parse()?],
    })
    .await
}

#[actix_rt::test]
async fn nested_exact() -> anyhow::Result<()> {
    get_nested_categories(&GetCategoryRequest {
        scope: None,
        ids: vec![
            "afbce03c-e90f-11ea-8281-cfde02f6b582".parse()?,
            "01cff7d8-e910-11ea-8281-7f86c625a156".parse()?,
        ],
    })
    .await
}

#[actix_rt::test]
async fn upgdate_ordering() -> anyhow::Result<()> {
    let category_three = "81c4796a-e883-11ea-93f0-df2484ab6b11";

    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/category/{}",
            port, category_three
        ))
        .json(&json!({"index": 0}))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/category", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/category/{}",
            port, category_three
        ))
        .json(&json!({"index": 2}))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/category", port))
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
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/category/7fe19326-e883-11ea-93f0-5343493c17c4",
            port,
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/category", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.updated_at" => "[timestamp]"});

    Ok(())
}

async fn update(id: Uuid, body: &serde_json::Value) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/category/{}", port, id))
        .json(&body)
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/category?scope=Decendants",
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
async fn update_parent() -> anyhow::Result<()> {
    update(
        "7fe19326-e883-11ea-93f0-5343493c17c4".parse()?,
        &json!({"parent_id": "81c4796a-e883-11ea-93f0-df2484ab6b11"}),
    )
    .await
}

#[actix_rt::test]
async fn update_reparent_move() -> anyhow::Result<()> {
    update(
        "7fe19326-e883-11ea-93f0-5343493c17c4".parse()?,
        &json!({"parent_id": (), "index": 0}),
    )
    .await
}

#[actix_rt::test]
async fn update_move() -> anyhow::Result<()> {
    update(
        "81c4796a-e883-11ea-93f0-df2484ab6b11".parse()?,
        &json!({"index": 1}),
    )
    .await
}

#[actix_rt::test]
async fn update_rename() -> anyhow::Result<()> {
    update(
        "81c4796a-e883-11ea-93f0-df2484ab6b11".parse()?,
        &json!({"name": "abc123"}),
    )
    .await
}
