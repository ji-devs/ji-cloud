use http::StatusCode;
use serde_json::json;
use shared::domain::{course::CourseId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn update_and_publish_browse() -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::MetaKinds,
            Fixture::User,
            Fixture::Jig,
            Fixture::Course,
        ],
        &[],
    )
    .await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d",
            port
        ))
        .json(&json!({
            "description": "asdasdasd",
            "language": "en-us",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/course/3a6a3660-f3ec-11ec-b8ef-071747fa2a0d/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/browse?authorId=1f241e1b-b537-493f-a230-075cb16315be&draftOrLive=draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        body, {
            // Really just need to redact the module ID because it is recreated for the live data,
            // but I couldn't get a selector working correctly... So redacting all IDs.
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]"
        }
    );

    app.stop(false).await;

    Ok(())
}
