use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::resource::curation::ResourceCurationCommentRequest;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Resource")
)]
async fn admin_comment(port: u16) -> anyhow::Result<()> {
    let name = "admin_comment";

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/curation",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body);

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/curation/comment",
            port
        ))
        .login()
        .json(&json!(ResourceCurationCommentRequest {
            value: "test comment".to_string()
        }))
        .send()
        .await?
        .error_for_status()?;

    let status = resp.status();

    assert_eq!(status, StatusCode::CREATED);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/curation",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2",name), body,  {
        ".**.id" => "[id]",
        ".**.createdAt" => "[created_at]",
    });

    Ok(())
}
