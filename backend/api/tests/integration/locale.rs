use http::StatusCode;
use macros::test_service;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod entry;

use crate::{fixture::Fixture, helpers::setup_service};

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Locale"))]
async fn list_bundles(port: u16) -> anyhow::Result<()> {
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

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Locale"))]
async fn list_item_kind(port: u16) -> anyhow::Result<()> {
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
