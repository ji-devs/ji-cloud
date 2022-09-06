use sqlx::PgPool;

mod animation;
mod audio;
mod auth;
mod category;
mod circle;
mod course;
mod fixture;
mod helpers;
mod image;
mod jig;
mod locale;
mod meta;
mod resource;
mod service;
mod session;
mod user;

#[sqlx::test]
async fn pass(pool: PgPool) -> anyhow::Result<()> {
    let app = helpers::initialize_server(&[], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    let resp = reqwest::get(&format!("http://0.0.0.0:{}", port)).await?;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

    Ok(())
}
