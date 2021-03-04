use http::StatusCode;

mod entry;

use crate::{fixture::Fixture, helpers::initialize_server};

#[actix_rt::test]
async fn list_bundles() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale]).await;

    let port = app.port();

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

#[actix_rt::test]
async fn list_item_kind() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Locale]).await;

    let port = app.port();

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
