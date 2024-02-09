use http::StatusCode;
use macros::test_service;
use serde_json::json;
use shared::domain::{course::CourseId, CreateResponse};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

pub mod unit;

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn get(port: u16) -> anyhow::Result<()> {
    let name = "get";

    let client = reqwest::Client::new();

    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{}/draft",
            port, course_id
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
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{}/live",
            port, course_id
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
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn update_and_publish_browse(port: u16) -> anyhow::Result<()> {
    let name = "update_and_publish_browse";

    let client = reqwest::Client::new();

    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{}/draft",
            port, course_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .patch(&format!("http://0.0.0.0:{}/v1/course/{}", port, course_id))
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
            "http://0.0.0.0:{}/v1/course/{}/draft",
            port, course_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{}/live",
            port, course_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-3",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/course/{}/draft/publish",
            port, course_id
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
        format!("{}-4",name),
        body, {
            // Really just need to redact the module ID because it is recreated for the live data,
            // but I couldn't get a selector working correctly... So redacting all IDs.
            ".**.id" => "[id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]"
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn browse_simple(port: u16) -> anyhow::Result<()> {
    let name = "browse_simple";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/course/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn live_up_to_date_flag(port: u16) -> anyhow::Result<()> {
    let name = "live_up_to_date_flag";

    let client = reqwest::Client::new();

    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-1",name),
        body, {
            ".**.lastEdited" => "[last_edited]",
        }
    );

    let _resp = client
        .put(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/draft/publish",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(
        format!("{}-2",name),
        body, {
            // Really just need to redact the module ID because it is recreated for the live data,
            // but I couldn't get a selector working correctly... So redacting all IDs.
            ".**.id" => "[id]",
            ".**.stable_id" => "[stable_id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.publishedAt" => "[published_at]",
            ".**.units" => "[units]",
        }
    );

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn clone(port: u16) -> anyhow::Result<()> {
    let name = "clone";
    let client = reqwest::Client::new();

    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/clone",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let CreateResponse { id: CourseId(id) } = resp.json().await?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/draft",
            port
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
            ".**.id" => "[id]",
            ".**.stable_id" => "[stable_id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.additionalResources" => "[ids]"
        }
    );

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/course/{}/draft", port, id))
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
            ".**.stable_id" => "[stable_id]",
            ".**.lastEdited" => "[last_edited]",
            ".**.feedbackPositive" => "[audio]",
            ".**.feedbackNegative" => "[audio]",
            ".**.additionalResources" => "[ids]"
        }
    );

    Ok(())
}
