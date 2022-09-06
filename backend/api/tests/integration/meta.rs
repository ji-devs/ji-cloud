use http::StatusCode;
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn get(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[
            Fixture::User,
            Fixture::Animation,
            Fixture::Image,
            Fixture::MetaKinds,
            Fixture::MetaImage,
            Fixture::MetaAnimation,
        ],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/metadata", port))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(body);

    Ok(())
}
