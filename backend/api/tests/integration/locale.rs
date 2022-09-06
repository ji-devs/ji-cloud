use http::StatusCode;
use sqlx::PgPool;

mod entry;

use crate::{fixture::Fixture, helpers::initialize_server};

#[sqlx::test]
async fn list_bundles(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/locale/bundle", port))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn list_item_kind(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/locale/item-kind", port))
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
