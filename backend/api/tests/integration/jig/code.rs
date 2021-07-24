use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::jig::code::{JigCodeResponse, JigIdFromCodeResponse};

#[actix_rt::test]
async fn create() -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::User, Fixture::Jig], &[]).await;

    let port = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/code",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: JigCodeResponse = resp.json().await?;

    insta::assert_json_snapshot!(body);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/code",
            port
        ))
        .login()
        .send()
        .await?;
    //.error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: JigCodeResponse = resp.json().await?;

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/jig/code", port))
        .json(&serde_json::json!({
            "code": body.code,
        }))
        .login()
        .send()
        .await?;
    //.error_for_status()?;

    app.stop(false).await;

    let body: JigIdFromCodeResponse = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
