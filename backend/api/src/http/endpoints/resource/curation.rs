use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::resource::curation, ApiEndpoint, PathParts},
    domain::{
        resource::{curation::CommentId, ResourceId},
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUserWithScope},
};

/// Update curation details for a Resource.
async fn update_curation(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<ResourceId>,
    req: Json<<curation::UpdateCuration as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let resource_id = path.into_inner();

    let req = req.into_inner();

    db::resource::curation::update(
        &*db,
        resource_id,
        req.display_name,
        req.categories,
        req.age_ranges,
        req.affiliations,
        req.language,
        req.description,
        req.additional_resources,
        req.curation_status,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get curation details for a Resource
async fn get_curation(
    db: Data<PgPool>,
    auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<ResourceId>,
) -> Result<Json<<curation::GetCuration as ApiEndpoint>::Res>, error::NotFound> {
    let resource_id = path.into_inner();
    let admin_id = auth.claims.user_id;

    db::resource::authz(&*db, admin_id, Some(resource_id)).await?;

    let curation = db::resource::curation::get_curation(&db, resource_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(curation))
}

/// Create a comment for resource curation.
async fn create_comment(
    db: Data<PgPool>,
    auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<ResourceId>,
    req: Json<<curation::CreateComment as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<curation::CreateComment as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let resource_id = path.into_inner();
    let admin_id = auth.claims.user_id;

    db::resource::authz(&*db, admin_id, Some(resource_id)).await?;

    let req = req.into_inner();

    let id =
        db::resource::curation::create_comment(&*db, resource_id, req.value, auth.claims.user_id)
            .await?;

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Get comment details for a resource curation
async fn get_comment(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<(ResourceId, CommentId)>,
) -> Result<Json<<curation::GetComment as ApiEndpoint>::Res>, error::NotFound> {
    let (resource_id, comment_id) = path.into_inner();

    let comment = db::resource::curation::get_comment(&db, resource_id, comment_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(comment))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <curation::UpdateCuration as ApiEndpoint>::Path::PATH,
        curation::UpdateCuration::METHOD.route().to(update_curation),
    )
    .route(
        <curation::GetCuration as ApiEndpoint>::Path::PATH,
        curation::GetCuration::METHOD.route().to(get_curation),
    )
    .route(
        <curation::CreateComment as ApiEndpoint>::Path::PATH,
        curation::CreateComment::METHOD.route().to(create_comment),
    )
    .route(
        <curation::GetComment as ApiEndpoint>::Path::PATH,
        curation::GetComment::METHOD.route().to(get_comment),
    );
}
