use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};
use http::StatusCode;
use macros::test_service;
use shared::domain::{
    course::unit::{CourseUnitCreateRequest, CourseUnitUpdateRequest, CourseUnitValue},
    image::ImageId,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[test_service(
    setup = "setup_service",
    fixtures(
        "Fixture::MetaKinds",
        "Fixture::User",
        "Fixture::Image",
        "Fixture::Course"
    )
)]
async fn create(port: u16) -> anyhow::Result<()> {
    let name = "create";
    let client = reqwest::Client::new();
    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/unit",
            port
        ))
        .login()
        .json(&CourseUnitCreateRequest {
            display_name: "image unit".to_string(),
            description: "create new unit".to_string(),
            value: CourseUnitValue::ImageId(ImageId(
                uuid::Uuid::parse_str("89125d88-ffaa-11eb-86a5-9fd50ab8d8df").unwrap(),
            )),
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}",name), body, {".id" => "[id]"});

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn update_index(port: u16) -> anyhow::Result<()> {
    let name = "update_index";
    let client = reqwest::Client::new();
    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();
    let course_unit_id = "e0984370-906c-11ed-b4f6-3f864931e86f".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-1", name), body);

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/unit/{course_unit_id}",
            port
        ))
        .login()
        .json(&CourseUnitUpdateRequest {
            display_name: None,
            description: None,
            value: None,
            index: Some(2),
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-2", name), body);

    let resp = client
        .patch(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/unit/{course_unit_id}",
            port
        ))
        .login()
        .json(&CourseUnitUpdateRequest {
            display_name: None,
            description: None,
            value: None,
            index: Some(1),
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}-3", name), body);
    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn get_draft(port: u16) -> anyhow::Result<()> {
    let name = "get_draft";
    let client = reqwest::Client::new();
    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();
    let course_unit_id = "e0984370-906c-11ed-b4f6-3f864931e86f".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/unit/{course_unit_id}/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn get_live(port: u16) -> anyhow::Result<()> {
    let name = "get_live";
    let client = reqwest::Client::new();
    let course_id = "47b3c062-906c-11ed-b4f6-9b0c5b1939a1".to_string();
    let course_unit_id = "09225312-906d-11ed-b4f6-afb0e90115b3".to_string();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/unit/{course_unit_id}/live",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}", name), body);

    Ok(())
}

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::Course")
)]
async fn delete(port: u16) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let course_id = "f77222a6-906b-11ed-b4f6-2f6dfab2ea0a".to_string();
    let course_unit_id = "e0984370-906c-11ed-b4f6-3f864931e86f".to_string();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/course/{course_id}/unit/{course_unit_id}/draft",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    Ok(())
}
