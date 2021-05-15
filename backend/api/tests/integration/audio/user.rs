use http::StatusCode;
use shared::domain::{audio::AudioId, CreateResponse};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[actix_rt::test]
async fn create_returns_created() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/audio", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: CreateResponse<AudioId> = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}
