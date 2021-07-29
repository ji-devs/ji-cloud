use http::{Method, StatusCode};
use serde_json::json;
use shared::error::{ApiError, EmptyError};

use crate::{
    fixture::Fixture,
    helpers::{initialize_server, LoginExt},
};

async fn forbidden(
    route: &str,
    req: Option<&serde_json::Value>,
    method: Method,
) -> anyhow::Result<()> {
    let app = initialize_server(&[Fixture::UserNoPerms], &[]).await;

    let port = app.port();

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

    app.stop(false).await;

    assert_eq!(body.code, StatusCode::FORBIDDEN);

    Ok(())
}

#[actix_rt::test]
async fn category_post() -> anyhow::Result<()> {
    forbidden("v1/category", Some(&json!({"name": ""})), Method::POST).await
}

#[actix_rt::test]
async fn category_patch() -> anyhow::Result<()> {
    forbidden(
        "v1/category/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
    )
    .await
}

#[actix_rt::test]
async fn category_delete() -> anyhow::Result<()> {
    forbidden(
        "v1/category/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
    )
    .await
}

#[actix_rt::test]
async fn image_post() -> anyhow::Result<()> {
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
                "kind": "Canvas",
        })),
        Method::POST,
    )
    .await
}

#[actix_rt::test]
async fn image_patch() -> anyhow::Result<()> {
    forbidden(
        "v1/image/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
    )
    .await
}

#[actix_rt::test]
#[ignore] // no s3
async fn image_delete() -> anyhow::Result<()> {
    forbidden(
        "v1/image/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
    )
    .await
}

#[actix_rt::test]
async fn jig_post() -> anyhow::Result<()> {
    forbidden("v1/jig", None, Method::POST).await
}

#[actix_rt::test]
async fn jig_patch() -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
    )
    .await
}

#[actix_rt::test]
async fn jig_clone() -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000/clone",
        None,
        Method::POST,
    )
    .await
}

// #[actix_rt::test]
// async fn jig_delete() -> anyhow::Result<()> {
//     forbidden(
//         "v1/jig/00000000-0000-0000-0000-000000000000",
//         None,
//         Method::DELETE,
//     )
//     .await
// }

#[actix_rt::test]
async fn module_post() -> anyhow::Result<()> {
    use shared::domain::jig::module::ModuleCreateRequest;

    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000/module",
        Some(&serde_json::to_value(ModuleCreateRequest::default())?),
        Method::POST,
    )
    .await
}

#[actix_rt::test]
async fn module_patch() -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000/module/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
    )
    .await
}

#[actix_rt::test]
async fn module_delete() -> anyhow::Result<()> {
    forbidden(
        "v1/jig/00000000-0000-0000-0000-000000000000/module/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
    )
    .await
}

#[actix_rt::test]
async fn animation_post() -> anyhow::Result<()> {
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
    )
    .await
}

#[actix_rt::test]
#[ignore] // route doesn't exist
async fn animation_patch() -> anyhow::Result<()> {
    forbidden(
        "v1/animation/00000000-0000-0000-0000-000000000000",
        None,
        Method::PATCH,
    )
    .await
}

#[actix_rt::test]
#[ignore] // no s3
async fn animation_delete() -> anyhow::Result<()> {
    forbidden(
        "v1/animation/00000000-0000-0000-0000-000000000000",
        None,
        Method::DELETE,
    )
    .await
}

// todo: admin routes
// todo: locale routes
