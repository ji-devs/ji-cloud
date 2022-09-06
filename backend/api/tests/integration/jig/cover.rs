use serde_json::json;
use shared::domain::jig::JigResponse;
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn update_no_modules_changes(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool,
    )
    .await;

    let port = app.port();

    let client = reqwest::Client::new();

    let _resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .json(&json! {{
            "displayName": "test",
        }})
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/jig/0cc084bc-7c83-11eb-9f77-e3218dffb008/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: JigResponse = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".**.lastEdited" => "[timestamp]", ".**.feedbackPositive" => "[audio]", ".**.feedbackNegative" => "[audio]"});

    Ok(())
}
