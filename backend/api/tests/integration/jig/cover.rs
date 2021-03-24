use serde_json::json;
use shared::domain::{
    jig::{JigId, JigResponse},
    CreateResponse,
};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create_default() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: CreateResponse<JigId> = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}", port, body.id.0))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: JigResponse = resp.json().await?;

    insta::assert_json_snapshot!(body.jig.modules, {"[].id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn create_with_cover() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Modules]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .json(&json! {{
            "display_name": (),
            "modules": ["5c8b6458-8840-11eb-8cf7-9bf404dc7386"],
            "content_types": [],
            "publish_at": (),

        }})
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: CreateResponse<JigId> = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}", port, body.id.0))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: JigResponse = resp.json().await?;

    insta::assert_json_snapshot!(body.jig.modules, {"[].id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn create_with_non_cover() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Modules]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/jig", port))
        .json(&json! {{
            "display_name": (),
            "modules": ["6dcd7cc4-8840-11eb-8cf7-3728f6dd85ce"],
            "content_types": [],
            "publish_at": (),

        }})
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: CreateResponse<JigId> = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/{}", port, body.id.0))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: JigResponse = resp.json().await?;

    insta::assert_json_snapshot!(body.jig.modules, {"[].id" => "[id]"});

    Ok(())
}

#[actix_rt::test]
async fn update_no_modules_changes() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let _resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .json(&json! {{
            "display_name": "test",

        }})
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: JigResponse = resp.json().await?;

    insta::assert_json_snapshot!(body.jig, {".**.updated_at" => "[updated_at]"});

    Ok(())
}
