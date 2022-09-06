use http::StatusCode;
use serde_json::json;
use shared::domain::resource::curation::ResourceCurationCommentRequest;
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn admin_comment(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::User, Fixture::MetaKinds, Fixture::Resource],
        &[],
        pool,
    )
    .await;

    let port = app.port();

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

    insta::assert_json_snapshot!(body);

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

    assert_eq!(resp.status(), StatusCode::CREATED);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{resource_id}/curation",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
        }
    );

    Ok(())
}
