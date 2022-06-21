use http::StatusCode;
use serde_json::json;
use shared::domain::{badge::BadgeId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/badge", port))
        .json(&json!({
            "displayName": "test badge",
            "description": "test description",
            "thumbnail": "https://www.jewishinteractive.org/wp-content/uploads/2022/05/create-reaction.png"
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<BadgeId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    let badge_id = body.id.0;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/badge/{}", port, badge_id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[actix_rt::test]
async fn delete() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/badge", port))
        .json(&json!({
            "displayName": "test badge",
            "description": "test description",
            "thumbnail": "https://www.jewishinteractive.org/wp-content/uploads/2022/05/create-reaction.png"
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<BadgeId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    let badge_id = body.id.0;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/badge/{}", port, badge_id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}
