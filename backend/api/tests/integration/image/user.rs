use http::StatusCode;
use macros::test_service;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

async fn list(query: &[(&str, &str)], name: &str, port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("http://0.0.0.0:{}/v1/user/me/image", port))
        .query(query)
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn list_kind(port: u16) -> anyhow::Result<()> {
    let name = "list_kind";

    list(&[("kind", "Sticker")], name, port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn list_all(port: u16) -> anyhow::Result<()> {
    let name = "list_all";

    list(&[], name, port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn create(port: u16) -> anyhow::Result<()> {
    let name = "create";
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("http://0.0.0.0:{}/v1/user/me/image", port))
        .json(&json!({
            "size": "Sticker",
        }))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body, {".**.id" => "[id]"});

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
async fn get(port: u16) -> anyhow::Result<()> {
    let name = "get";
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/user/me/image/{}",
            port, "89125d88-ffaa-11eb-86a5-9fd50ab8d8df"
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}",name), body, {".**.id" => "[id]"});

    Ok(())
}

// needs s3
// #[ignore]
// #[test_service(setup = "setup_service", fixtures("Fixture::User", "Fixture::Image"))]
// async fn delete(port: u16) -> anyhow::Result<()> {
//     let name = "delete";
//     let client = reqwest::Client::new();

//     let resp = client
//         .delete(&format!(
//             "http://0.0.0.0:{}/v1/user/me/image/{}",
//             port, "89125d88-ffaa-11eb-86a5-9fd50ab8d8df"
//         ))
//         .login()
//         .send()
//         .await?
//         .error_for_status()?;

//     assert_eq!(resp.status(), StatusCode::CREATED);

//     let body: serde_json::Value = resp.json().await?;

//     insta::assert_json_snapshot!(format!("{}",name), body, {".**.id" => "[id]"});

//     Ok(())
// }
