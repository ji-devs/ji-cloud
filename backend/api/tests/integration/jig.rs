use http::StatusCode;
use shared::domain::{jig::JigId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create_default() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

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

    Ok(())
}

// todo: create_params
// todo: delete
// todo: get
// todo: update
