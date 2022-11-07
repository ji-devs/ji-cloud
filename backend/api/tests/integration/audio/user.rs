use http::StatusCode;
use macros::test_service;
use shared::domain::{audio::AudioId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(setup = "setup_service", fixtures("Fixture::User"))]
async fn create_returns_created(port: u16) -> anyhow::Result<()> {
    let name = "created_returns_created";
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/audio", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<AudioId> = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}",name), body, {".id" => "[id]"});

    Ok(())
}
