use crate::helpers::setup_service;
use macros::test_service;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

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
mod pro_dev;
mod resource;
mod service;
mod session;
mod user;

#[test_service(setup = "setup_service")]
async fn pass(port: u16) -> anyhow::Result<()> {
    let resp = reqwest::get(&format!("http://0.0.0.0:{}", port)).await?;

    assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

    Ok(())
}
