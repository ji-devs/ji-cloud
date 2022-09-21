use serde_json::json;
use shared::domain::resource::ResourceResponse;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

#[sqlx::test]
async fn update_no_modules_changes(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Resource],
        &[],
        pool_opts,
        conn_opts,
    )
    .await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resource_id = "d8067526-1518-11ed-87fa-ebaf880b6d9c".to_string();

    let _resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/resource/{}",
            port, resource_id
        ))
        .json(&json! {{
            "displayName": "test Resource",
        }})
        .login()
        .send()
        .await?
        .error_for_status()?;

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/resource/{}/draft",
            port, resource_id
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    let body: ResourceResponse = resp.json().await?;

    insta::assert_json_snapshot!(body, {".**.lastEdited" => "[timestamp]"});

    Ok(())
}
