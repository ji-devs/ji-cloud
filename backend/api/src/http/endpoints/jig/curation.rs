use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::jig::curation, ApiEndpoint},
    domain::jig::JigId,
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUser, TokenUserWithScope},
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
    auth: TokenUser,
    path: Path<JigId>,
) -> Result<Json<<curation::GetCuration as ApiEndpoint>::Res>, error::NotFound> {
    let jig_id = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(jig_id)).await?;

    let curation = db::jig::curation::get_curation(&db, jig_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(curation))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        curation::UpdateCuration::PATH,
        curation::UpdateCuration::METHOD.route().to(update_curation),
    )
    .route(
        curation::GetCuration::PATH,
        curation::GetCuration::METHOD.route().to(get_curation),
    );
}
