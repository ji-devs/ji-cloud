use http::StatusCode;

use serde_json::json;
use shared::domain::locale::{CreateEntryRequest, EntryStatus};
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn delete(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!("http://0.0.0.0:{}/v1/locale/entry/3", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[sqlx::test]
async fn get(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/locale/entry/3", port))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

async fn list(query: &[(&str, &str)], pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/locale/entry", port))
        .query(query)
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn list_all_by_default(pool: PgPool) -> anyhow::Result<()> {
    list(&[], pool).await
}

#[sqlx::test]
async fn list_all_by_bundle(pool: PgPool) -> anyhow::Result<()> {
    list(&[("groupBy", "bundle")], pool).await
}

#[sqlx::test]
async fn list_empty_bundle_by_default(pool: PgPool) -> anyhow::Result<()> {
    list(&[("bundles", "85a46ffe-7c67-11eb-a0d7-277d94fe130c")], pool).await
}

#[sqlx::test]
async fn list_empty_bundle_by_bundle(pool: PgPool) -> anyhow::Result<()> {
    list(
        &[
            ("groupBy", "bundle"),
            ("bundles", "85a46ffe-7c67-11eb-a0d7-277d94fe130c"),
        ],
        pool,
    )
    .await
}

#[sqlx::test]
async fn list_single_bundle_by_default(pool: PgPool) -> anyhow::Result<()> {
    list(&[("bundles", "8359a48a-7c67-11eb-a0d7-0fd74777a62c")], pool).await
}

#[sqlx::test]
async fn list_single_bundle_by_bundle(pool: PgPool) -> anyhow::Result<()> {
    list(
        &[
            ("groupBy", "bundle"),
            ("bundles", "8359a48a-7c67-11eb-a0d7-0fd74777a62c"),
        ],
        pool,
    )
    .await
}

#[sqlx::test]
async fn create(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/locale/entry", port))
        .json(&CreateEntryRequest {
            bundle_id: "8359a48a-7c67-11eb-a0d7-0fd74777a62c".parse()?,
            section: None,
            item_kind_id: None,
            english: Some("Just another test entry".to_owned()),
            hebrew: None,
            status: EntryStatus::Discuss,
            zeplin_reference: None,
            comments: None,
            in_app: false,
            in_element: true,
            in_mock: true,
        })
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[sqlx::test]
async fn update_in_app(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/locale/entry/3", port))
        .json(&json! ({
            "inApp": false,
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/locale/entry", port))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}
