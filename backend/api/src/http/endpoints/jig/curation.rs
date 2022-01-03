use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::jig::curation, ApiEndpoint},
    domain::{
        jig::{curation::CommentId, JigId},
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUserWithScope},
};

/// Update curation details for a Jig.
async fn update_curation(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<JigId>,
    req: Json<<curation::UpdateCuration as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let jig_id = path.into_inner();

    let req = req.into_inner();

    db::jig::curation::update(
        &*db,
        jig_id,
        req.display_name,
        req.goals,
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

/// Get curation details for a Jig
async fn get_curation(
    db: Data<PgPool>,
    auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<JigId>,
) -> Result<Json<<curation::GetCuration as ApiEndpoint>::Res>, error::NotFound> {
    let jig_id = path.into_inner();

    db::jig::authz(&*db, auth.claims.user_id, Some(jig_id)).await?;

    let curation = db::jig::curation::get_curation(&db, jig_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(curation))
}

/// Create a new additional resource.
async fn create_comment(
    db: Data<PgPool>,
    auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<JigId>,
    req: Json<<curation::CreateComment as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<curation::CreateComment as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let jig_id = path.into_inner();

    db::jig::authz(&*db, auth.claims.user_id, Some(jig_id)).await?;

    let req = req.into_inner();

    let id =
        db::jig::curation::create_comment(&*db, jig_id, req.value, auth.claims.user_id).await?;

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Get curation details for a Jig
async fn get_comment(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    path: Path<(JigId, CommentId)>,
) -> Result<Json<<curation::GetComment as ApiEndpoint>::Res>, error::NotFound> {
    let (jig_id, comment_id) = path.into_inner();

    let comment = db::jig::curation::get_comment(&db, jig_id, comment_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(comment))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        curation::UpdateCuration::PATH,
        curation::UpdateCuration::METHOD.route().to(update_curation),
    )
    .route(
        curation::GetCuration::PATH,
        curation::GetCuration::METHOD.route().to(get_curation),
    )
    .route(
        curation::CreateComment::PATH,
        curation::CreateComment::METHOD.route().to(create_comment),
    )
    .route(
        curation::GetComment::PATH,
        curation::GetComment::METHOD.route().to(get_comment),
    );
}
