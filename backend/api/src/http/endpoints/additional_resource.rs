use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Path, ServiceConfig},
    CreatedJson, NoContent,
};
use shared::{
    api::{endpoints::jig::additional_resource, ApiEndpoint},
    domain::{
        jig::{additional_resource::AdditionalResourceId, JigId},
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};
use shared::domain::jig::additional_resource::AdditionalResourceResponse;

/// Create a new additional resource.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    parent: Path<JigId>,
    req: Json<<additional_resource::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<additional_resource::Create as ApiEndpoint>::Res>, error::Auth> {
    let parent_id = parent.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let req = req.into_inner();

    let id = db::additional_resource::create(&*db, parent_id, req.url).await?;

    Ok(CreatedJson(CreateResponse { id }))
}

/// Get an additional resource.
#[api_v2_operation]
async fn get(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<(JigId, AdditionalResourceId)>,
) -> Result<Json<<additional_resource::Get as ApiEndpoint>::Res>, error::NotFound> {
    let (parent_id, additional_resource_id) = path.into_inner();

    let url = db::additional_resource::get(&db, parent_id, additional_resource_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(AdditionalResourceResponse { url }))
}

/// Update an additional resource.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<(JigId, AdditionalResourceId)>,
    url: Option<String>,
) -> Result<NoContent, error::NotFound> {
    let (parent_id, additional_resource_id) = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let exists =
        db::additional_resource::update(&*db, parent_id, additional_resource_id, url).await?;

    match exists {
        true => Ok(NoContent),
        false => Err(error::NotFound::ResourceNotFound),
    }
}

/// Delete an additional resource.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<(JigId, AdditionalResourceId)>,
) -> Result<NoContent, error::Delete> {
    let (parent_id, additional_resource_id) = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    db::additional_resource::delete(&*db, parent_id, additional_resource_id).await?;

    Ok(NoContent)
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        additional_resource::Create::PATH,
        additional_resource::Create::METHOD.route().to(create),
    )
    .route(
        additional_resource::Get::PATH,
        additional_resource::Get::METHOD.route().to(get),
    )
    .route(
        additional_resource::Update::PATH,
        additional_resource::Update::METHOD.route().to(update),
    )
    .route(
        additional_resource::Delete::PATH,
        additional_resource::Delete::METHOD.route().to(delete),
    );
}
