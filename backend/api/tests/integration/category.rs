use http::StatusCode;
use shared::domain::category::{CategoryTreeScope, GetCategoryRequest};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::CategoryOrdering]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

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

    let _ = tokio::spawn(app.run_until_stopped());

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
