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

    app.stop(false).await;

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
async fn join_badge() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Badge], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/badge/{}/members",
            port, "a3126bec-f185-11ec-b9e4-5fa4e257b5a1"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/badge/{}/join",
            port, "a3126bec-f185-11ec-b9e4-5fa4e257b5a1"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/badge/{}/members",
            port, "a3126bec-f185-11ec-b9e4-5fa4e257b5a1"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn leave_badge() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Badge], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/badge/{}",
            port, "829606d0-f185-11ec-b9e4-5fadfd7252f6"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/badge/{}/leave",
            port, "829606d0-f185-11ec-b9e4-5fadfd7252f6"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/badge/{}",
            port, "829606d0-f185-11ec-b9e4-5fadfd7252f6"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[actix_rt::test]
async fn browse_badges() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Badge], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/badge/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    app.stop(false).await;

    Ok(())
}

#[actix_rt::test]
async fn update_badge() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Badge], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/badge/{}",
            port, "57a1eaaa-f182-11ec-a96e-13f3929f5b22"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/badge/{}",
            port, "57a1eaaa-f182-11ec-a96e-13f3929f5b22"
        ))
        .json(&json!({
            "displayName": "update badge",
            "description": "update description",
            "thumbnail": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR0VKBsgtcc6vaIjsj5g9T41vLZsqBXYre_MQ&usqp=CAU"
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/badge/{}",
            port, "57a1eaaa-f182-11ec-a96e-13f3929f5b22"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.createdAt" => "[created_at]",
            ".**.lastEdited" => "[last_edited]",
        }
    );

    app.stop(false).await;

    Ok(())
}
