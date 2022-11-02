use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::{circle::CircleId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn create(port: u16) -> anyhow::Result<()> {
    let name = "create";
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/circle", port))
        .json(&json!({
            "displayName": "test circle",
            "description": "test description",
            "image": "8a473dd6-ffaa-11eb-86a5-dba3538e5a15"
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<CircleId> = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1",name), body, {".id" => "[id]"});

    let circle_id = body.id.0;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/circle/{}", port, circle_id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.id" => "[id]",
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::Circle")
)]
async fn join_circle(port: u16) -> anyhow::Result<()> {
    let name = "join_circle";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/{}/members",
            port, "a3126bec-f185-11ec-b9e4-5fa4e257b5a1"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/circle/{}/join",
            port, "a3126bec-f185-11ec-b9e4-5fa4e257b5a1"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/{}/members",
            port, "a3126bec-f185-11ec-b9e4-5fa4e257b5a1"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::Circle")
)]
async fn leave_circle(port: u16) -> anyhow::Result<()> {
    let name = "leave_circle";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/{}",
            port, "829606d0-f185-11ec-b9e4-5fadfd7252f6"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/circle/{}/leave",
            port, "829606d0-f185-11ec-b9e4-5fadfd7252f6"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/{}",
            port, "829606d0-f185-11ec-b9e4-5fadfd7252f6"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}
#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::Circle")
)]
async fn browse_circles(port: u16) -> anyhow::Result<()> {
    let name = "browse_circle";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/circle/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::Circle")
)]
async fn update_circle(port: u16) -> anyhow::Result<()> {
    let name = "update_circle";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/{}",
            port, "57a1eaaa-f182-11ec-a96e-13f3929f5b22"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/circle/{}",
            port, "57a1eaaa-f182-11ec-a96e-13f3929f5b22"
        ))
        .json(&json!({
            "displayName": "update circle",
            "description": "update description",
            "image": "8a473dd6-ffaa-11eb-86a5-dba3538e5a15"
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/{}",
            port, "57a1eaaa-f182-11ec-a96e-13f3929f5b22"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::User", "Fixture::Image", "Fixture::Circle")
)]
async fn browse_circles_with_users(port: u16) -> anyhow::Result<()> {
    let name = "browse_circles_with_users";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/circle/browse?users=7b96a41c-e406-11eb-8176-efd86dd7f444",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}",name),
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}
