use http::StatusCode;
use serde_json::json;
use shared::domain::category::{
    CategoryTreeScope, CreateCategoryRequest, GetCategoryRequest, NewCategoryResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn create(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn get(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

async fn get_nested_categories(query: &GetCategoryRequest, pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryNesting], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn nested_top_level(pool: PgPool) -> anyhow::Result<()> {
    get_nested_categories(&GetCategoryRequest::default(), pool).await
}

#[sqlx::test]
async fn nested_whole_tree(pool: PgPool) -> anyhow::Result<()> {
    get_nested_categories(
        &GetCategoryRequest {
            scope: Some(CategoryTreeScope::Descendants),
            ids: vec![],
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn nested_overlapping(pool: PgPool) -> anyhow::Result<()> {
    get_nested_categories(
        &GetCategoryRequest {
            scope: Some(CategoryTreeScope::Descendants),
            ids: vec![
                "afbce03c-e90f-11ea-8281-cfde02f6b582".parse()?,
                "e315d3b2-e90f-11ea-8281-73cd69c14821".parse()?,
            ],
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn nested_ancestors(pool: PgPool) -> anyhow::Result<()> {
    get_nested_categories(
        &GetCategoryRequest {
            scope: Some(CategoryTreeScope::Ancestors),
            ids: vec!["e315d3b2-e90f-11ea-8281-73cd69c14821".parse()?],
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn nested_exact(pool: PgPool) -> anyhow::Result<()> {
    get_nested_categories(
        &GetCategoryRequest {
            scope: None,
            ids: vec![
                "afbce03c-e90f-11ea-8281-cfde02f6b582".parse()?,
                "01cff7d8-e910-11ea-8281-7f86c625a156".parse()?,
            ],
        },
        pool,
    )
    .await
}

#[sqlx::test]
async fn upgdate_ordering(pool: PgPool) -> anyhow::Result<()> {
    let category_three = "81c4796a-e883-11ea-93f0-df2484ab6b11";

    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

#[sqlx::test]
async fn delete(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

async fn update(id: Uuid, body: &serde_json::Value, pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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
            "http://0.0.0.0:{}/v1/category?scope=Descendants",
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

#[sqlx::test]
async fn update_parent(pool: PgPool) -> anyhow::Result<()> {
    update(
        "7fe19326-e883-11ea-93f0-5343493c17c4".parse()?,
        &json!({"parent_id": "81c4796a-e883-11ea-93f0-df2484ab6b11"}),
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_reparent_move(pool: PgPool) -> anyhow::Result<()> {
    update(
        "7fe19326-e883-11ea-93f0-5343493c17c4".parse()?,
        &json!({"parent_id": (), "index": 0}),
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_move(pool: PgPool) -> anyhow::Result<()> {
    update(
        "81c4796a-e883-11ea-93f0-df2484ab6b11".parse()?,
        &json!({"index": 1}),
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_scope(pool: PgPool) -> anyhow::Result<()> {
    update(
        "81c4796a-e883-11ea-93f0-df2484ab6b11".parse()?,
        &json!({"user_scopes": ["Admin", "ManageCategory", "ManageImage", "ManageAnimation"]}),
        pool,
    )
    .await
}

#[sqlx::test]
async fn update_rename(pool: PgPool) -> anyhow::Result<()> {
    update(
        "81c4796a-e883-11ea-93f0-df2484ab6b11".parse()?,
        &json!({"name": "abc123"}),
        pool,
    )
    .await
}
