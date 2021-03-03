mod auth;
mod category;
mod fixture;
mod helpers;
mod image;
mod jig;
mod meta;
mod session;
mod user;

#[actix_rt::test]
async fn pass() -> anyhow::Result<()> {
    let app = helpers::initialize_server(&[]).await;

    let port = app.port();

    let _ = tokio::spawn(app.run_until_stopped());

    let resp = reqwest::get(&format!("http://0.0.0.0:{}", port)).await?;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

    Ok(())
}

// todo: locale tests
