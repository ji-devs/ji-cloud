use http::StatusCode;
use shared::error::{ApiError, EmptyError};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{fixture::Fixture, helpers::initialize_server};

async fn unauthorized(
    route: &str,
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserNoPerms], &[], pool_opts, conn_opts).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/{}", port, route))
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let body: ApiError<EmptyError> = resp.json().await?;

    assert_eq!(body.code, StatusCode::UNAUTHORIZED);

    Ok(())
}

#[sqlx::test]
async fn get_image(pool_opts: PgPoolOptions, conn_opts: PgConnectOptions) -> anyhow::Result<()> {
    unauthorized(
        "v1/image/00000000-0000-0000-0000-000000000000",
        pool_opts,
        conn_opts,
    )
    .await
}

#[sqlx::test]
async fn get_animation(
    pool_opts: PgPoolOptions,
    conn_opts: PgConnectOptions,
) -> anyhow::Result<()> {
    unauthorized(
        "v1/animation/00000000-0000-0000-0000-000000000000",
        pool_opts,
        conn_opts,
    )
    .await
}
