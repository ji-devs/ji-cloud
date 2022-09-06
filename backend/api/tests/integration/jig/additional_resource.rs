use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
use http::StatusCode;
use shared::domain::{
    additional_resource::{AdditionalResourceCreateRequest, AssetIdResource, ResourceContent},
    asset::AssetId,
    image::ImageId,
    jig::JigId,
    meta::ResourceTypeId,
};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

#[sqlx::test]
async fn create(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool,
    )
    .await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .post(&format!(
            "http://0.0.0.0:{}/v1/additional-resource/draft",
            port
        ))
        .login()
        .json(&AdditionalResourceCreateRequest {
            asset_id: AssetId::JigId(JigId(
                uuid::Uuid::parse_str("0cc084bc-7c83-11eb-9f77-e3218dffb008").unwrap(),
            )),
            display_name: "testing".to_string(),
            resource_type_id: ResourceTypeId(
                Uuid::from_str("a939f454-519e-11ec-ab46-2fa68cd3a8c7").unwrap(),
            ),
            resource_content: ResourceContent::ImageId(ImageId(
                Uuid::from_str("a974ce0e-ef6e-11eb-ad5a-bf4be1413928").unwrap(),
            )),
        })
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body, {".id" => "[id]"});

    Ok(())
}

#[sqlx::test]
async fn get_draft(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool,
    )
    .await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/additional-resource/286b8390-1dd9-11ec-8426-fbeb80c504d9/draft?jigId=0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn get_live(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool,
    )
    .await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/additional-resource/286b828c-1dd9-11ec-8426-571b03b2d3df/live?jigId=0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    app.stop(false).await;

    insta::assert_json_snapshot!(body);

    Ok(())
}

#[sqlx::test]
async fn delete(pool: PgPool) -> anyhow::Result<()> {
    let app = initialize_server(
        &[Fixture::MetaKinds, Fixture::User, Fixture::Jig],
        &[],
        pool,
    )
    .await;

    let port: u16 = app.port();

    let client = reqwest::Client::new();

    let resp = client
        .delete(&format!(
            "http://0.0.0.0:{}/v1/additional-resource/286b8390-1dd9-11ec-8426-fbeb80c504d9/draft?jigId=0cc084bc-7c83-11eb-9f77-e3218dffb008",
            port
        ))
        .json(&AssetIdResource {
            asset_id: Some(AssetId::JigId(JigId(
                uuid::Uuid::parse_str("0cc084bc-7c83-11eb-9f77-e3218dffb008").unwrap()))),
})
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    app.stop(false).await;

    Ok(())
}
