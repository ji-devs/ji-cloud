use http::StatusCode;
use macros::test_service;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::User",
        "Fixture::Animation",
        "Fixture::Image",
        "Fixture::MetaKinds",
        "Fixture::MetaImage",
        "Fixture::MetaAnimation"
    )
)]
async fn get(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/metadata", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
