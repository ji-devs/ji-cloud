use macros::test_service;
use serde_json::json;
use shared::domain::jig::JigResponse;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Jig")
)]
async fn update_no_modules_changes(port: u16) -> anyhow::Result<()> {
    let name = "update_no_modules_changes";
    let client = reqwest::Client::new();

    let _resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .json(&json! {{
            "displayName": "test",
        }})
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: JigResponse = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body, {".**.lastEdited" => "[timestamp]", ".**.feedbackPositive" => "[audio]", ".**.feedbackNegative" => "[audio]"});

    Ok(())
}
