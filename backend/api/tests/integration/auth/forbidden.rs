use http::{Method, StatusCode};
use serde_json::json;
use shared::error::{ApiError, EmptyError};
use sqlx::PgPool;

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};
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
    pool: PgPool,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserNoPerms], &[], pool).await;

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

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

    let body: ApiError<EmptyError> = resp.json().await?;

    assert_eq!(body.code, StatusCode::FORBIDDEN);

    Ok(())
}

#[sqlx::test]
async fn category_post(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/category",
        Some(&json!({"name": ""})),
        Method::POST,
        pool,
    )
    .await
}

#[sqlx::test]
async fn category_patch(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/category/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        pool,
    )
    .await
}

#[sqlx::test]
async fn category_delete(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/category/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
        pool,
    )
    .await
}

#[sqlx::test]
async fn image_post(pool: PgPool) -> anyhow::Result<()> {
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
        pool,
    )
    .await
}

#[sqlx::test]
async fn image_patch(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/image/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        pool,
    )
    .await
}

#[sqlx::test]
#[ignore] // no s3
async fn image_delete(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/image/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
        pool,
    )
    .await
}

#[sqlx::test]
async fn jig_post(pool: PgPool) -> anyhow::Result<()> {
    forbidden("v1/jig", None, Method::POST, pool).await
}

#[sqlx::test]
async fn jig_patch(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        pool,
    )
    .await
}

#[sqlx::test]
async fn jig_clone(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000/clone",
        None,
        Method::POST,
        pool,
    )
    .await
}

// #[sqlx::test]
// async fn jig_delete(pool: PgPool) -> anyhow::Result<()> {
//     forbidden(
//         "v1/jig/00000000-0000-0000-0000-000000000000",
//         None,
//         Method::DELETE,
//     )
//     .await
// }

#[sqlx::test]
async fn module_post(pool: PgPool) -> anyhow::Result<()> {
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
        pool,
    )
    .await
}

#[sqlx::test]
async fn module_patch(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/module/draft/00000000-0000-0000-0000-000000000000",
        Some(&serde_json::json!({"jigId": "00000000-0000-0000-0000-000000000000"})),
        Method::PATCH,
        pool,
    )
    .await
}

#[sqlx::test]
async fn module_delete(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/module/draft/00000000-0000-0000-0000-000000000000",
        Some(&serde_json::json!({"jigId": "00000000-0000-0000-0000-000000000000"})),
        Method::DELETE,
        pool,
    )
    .await
}

#[sqlx::test]
async fn animation_post(pool: PgPool) -> anyhow::Result<()> {
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
        pool,
    )
    .await
}

#[sqlx::test]
#[ignore] // route doesn't exist
async fn animation_patch(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/animation/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
        pool,
    )
    .await
}

#[sqlx::test]
#[ignore] // no s3
async fn animation_delete(pool: PgPool) -> anyhow::Result<()> {
    forbidden(
        "v1/animation/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
        pool,
    )
    .await
}

// todo: admin routes
// todo: locale routes
