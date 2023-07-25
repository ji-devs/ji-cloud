use http::{Method, StatusCode};
use macros::test_service;
use serde_json::json;
use shared::error::EmptyError;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::{
    fixture::Fixture,
    helpers::{setup_service, LoginExt},
};
use ji_cloud_api::error::ApiResponseError;
use shared::domain::{
    asset::AssetId,
    jig::JigId,
    module::{ModuleBody, ModuleKind},
};
use uuid::Uuid;

async fn forbidden(
    route: &str,
    req: Option<&serde_json::Value>,
    method: Method,
    port: u16,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let request = client
        .request(method, &format!("http://0.0.0.0:{}/{}", port, route))
        .login();

    let request = if let Some(req) = req {
        request.json(&req)
    } else {
        request
    };

    let resp = request.send().await?;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let body: ApiResponseError<EmptyError> = resp.json().await?;

    assert_eq!(body.code, StatusCode::FORBIDDEN);

    Ok(())
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn category_post(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/category",
        Some(&json!({"name": "Fixture::UserNoPerms"})),
        Method::POST,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn category_patch(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/category/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn category_delete(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/category/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn image_post(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/image",
        Some(&json!({
                "name": "test",
                "description": "testest",
                "is_premium": false,
                "publish_at": (),
                "styles": [],
                "age_ranges": [],
                "affiliations": [],
                "categories": [],
                "tags": [],
                "size": "Canvas",
        })),
        Method::POST,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn image_patch(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/image/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn jig_post(port: u16) -> anyhow::Result<()> {
    forbidden("v1/jig", None, Method::POST, port).await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn jig_patch(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn jig_clone(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000/clone",
        None,
        Method::POST,
        port,
    )
    .await
}

// #[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
// async fn jig_delete(pool_opts: PoolOptions<Postgres>, conn_opts: PgConnectOptions<Postgres>) -> anyhow::Result<()> {
//     forbidden(
//         "v1/jig/00000000-0000-0000-0000-000000000000",
//         None,
//         Method::DELETE,
//     )
//     .await
// }

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn module_post(port: u16) -> anyhow::Result<()> {
    use shared::domain::module::ModuleCreateRequest;

    forbidden(
        "v1/module/draft",
        Some(&serde_json::to_value(ModuleCreateRequest {
            parent_id: AssetId::JigId(JigId(Uuid::parse_str(
                "00000000-0000-0000-0000-000000000000",
            )?)),
            body: ModuleBody::new(ModuleKind::Cover),
        })?),
        Method::POST,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn module_patch(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/module/draft/00000000-0000-0000-0000-000000000000",
        Some(&serde_json::json!({"jigId": "00000000-0000-0000-0000-000000000000"})),
        Method::PATCH,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn module_delete(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/module/draft/00000000-0000-0000-0000-000000000000",
        Some(&serde_json::json!({"jigId": "00000000-0000-0000-0000-000000000000"})),
        Method::DELETE,
        port,
    )
    .await
}

#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
async fn animation_post(port: u16) -> anyhow::Result<()> {
    forbidden(
        "v1/animation",
        Some(&json!({
            "name": "test",
            "description": "testest",
            "is_premium": false,
            "publish_at": (),
            "styles": [],
            "is_looping": false,
            "kind": "Gif",
        })),
        Method::POST,
        port,
    )
    .await
}

// Ignored tests aren't captured. Will resolve later
//
// #[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
// #[ignore] // route doesn't exist
// async fn animation_patch(port: u16) -> anyhow::Result<()> {
//     forbidden(
//         "v1/animation/00000000-0000-0000-0000-000000000000",
//         None,
//         Method::PATCH,
//         port,
//     )
//     .await
// }

// #[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
// #[ignore] // no s3
// async fn animation_delete(port: u16) -> anyhow::Result<()> {
//     forbidden(
//         "v1/animation/00000000-0000-0000-0000-000000000000",
//         None,
//         Method::DELETE,
//         port,
//     )
//     .await
// }
//
//#[test_service(setup = "setup_service", fixtures("Fixture::UserNoPerms"))]
// #[ignore] // no s3
// async fn image_delete(port: u16) -> anyhow::Result<()> {
//     forbidden(
//         "v1/image/00000000-0000-0000-0000-000000000000",
//         None,
//         Method::DELETE,
//         port,
//     )
//     .await
// }

// todo: admin routes
// todo: locale routes
