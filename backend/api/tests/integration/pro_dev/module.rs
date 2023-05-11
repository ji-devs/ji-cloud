use http::StatusCode;

use macros::test_service;
use serde_json::json;
use shared::domain::{
    asset::{AssetId, AssetType},
    module::{body::memory, ModuleBody, ModuleCreateRequest, ModuleKind, ModuleUpdateRequest},
    pro_dev::ProDevId,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use uuid::Uuid;

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};

#[test_service(
    setup = "setup_service",
    fixtures("Fixture::MetaKinds", "Fixture::User", "Fixture::ProDev")
)]
async fn get_draft(port: u16) -> anyhow::Result<()> {
    let name = "get_draft";

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!(
            "http://0.0.0.0:{}/v1/{}/module/draft/3a272c30-906d-11ed-b4f6-1fef9f7412be",
            port,
            AssetType::ProDev
        ))
        .login()
        .send()
        .await?
        .error_for_status()?;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: serde_json::Value = resp.json().await?;

    insta::assert_json_snapshot!(format!("{}",name), body, {".**.updated_at" => "[timestamp]"});

    Ok(())
}
