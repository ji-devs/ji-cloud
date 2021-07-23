use http::StatusCode;
use serde_json::json;
use shared::domain::{jig::JigId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

mod additional_resource;
mod cover;
mod draft;
mod module;

#[actix_rt::test]
async fn create_default() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<JigId> = resp.json().await?;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}", port, body.id.0))
        .login()
        .send()
        .await?
        .error_for_status()?;

    app.stop(false).await;

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.id" => "[id]", ".*.last_edited" => "[time_stamp]"});

    Ok(())
}

// #[actix_rt::test]
// async fn delete() -> anyhow::Result<()> {
//     let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

//     let port = app.port();

//     let client = reqwest::Client::new();

//     let resp = client
//         .delete(&format!(
//             "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
//             port
//         ))
//         .login()
//         .send()
//         .await?
//         .error_for_status()?;

//     assert_eq!(resp.status(), StatusCode::NO_CONTENT);

//     Ok(())
// }

#[actix_rt::test]
async fn create_with_params() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .json(&json!({
            "modules": ["0cbfdd82-7c83-11eb-9f77-d7d86264c3bc"],
            "display_name": "test jig",
            "description": "test description",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<JigId> = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]", ".last_edited" => "[last_edited]"});

    Ok(())
}

#[actix_rt::test]
async fn clone() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/clone",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let CreateResponse { id: JigId(id) } = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}", port, id))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(
        body, {
            ".**.id" => "[id]",
            ".**.last_edited" => "[last_edited]",
            ".**.feedback_positive" => "[audio]",
            ".**.feedback_negative" => "[audio]",
            ".**.additional_resources" => "[ids]"
        }
    );

    Ok(())
}

#[actix_rt::test]
async fn get() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(
        body, {
            ".**.last_edited" => "[last_edited]",
            ".**.feedback_positive" => "[audio]",
            ".**.feedback_negative" => "[audio]"
        }
    );

    Ok(())
}

// todo: test-exhaustiveness: create a `JigBrowse` Fixture, actually test the cases (paging, jig count, etc)
#[actix_rt::test]
async fn browse_simple() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/browse", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(
        body, {
            ".**.last_edited" => "[last_edited]",
            ".**.feedback_positive" => "[audio]",
            ".**.feedback_negative" => "[audio]"
        }
    );

    Ok(())
}

// todo: test-exhaustiveness: create a `JigBrowse` Fixture, actually test the cases (paging, jig count, etc)
#[actix_rt::test]
async fn browse_own_simple() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserDefaultPerms, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/browse?authorId=me",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(
        body, {
            ".**.last_edited" => "[last_edited]",
            ".**.feedback_positive" => "[audio]",
            ".**.feedback_negative" => "[audio]"
        }
    );

    Ok(())
}

#[actix_rt::test]
async fn count() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserDefaultPerms, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/count", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}
