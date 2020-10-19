use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::jig::CreateResponse,
    domain::jig::GetResponse,
    domain::jig::JigId,
    error::jig::UpdateError,
    error::GetError,
};
use sqlx::PgPool;

use crate::{
    db,
    extractor::{AuthUserWithScope, ScopeManageJig, WrapAuthClaimsNoDb},
};

async fn create(
    db: Data<PgPool>,
    auth: AuthUserWithScope<ScopeManageJig>,
    Json(req): Json<<jig::Create as ApiEndpoint>::Req>,
) -> Result<Json<<jig::Create as ApiEndpoint>::Res>, <jig::Create as ApiEndpoint>::Err> {
    let creator_id = auth.claims.id;
    let id = db::jig::create(
        &*db,
        &req.display_name,
        &req.cover,
        &req.modules,
        &req.ending,
        creator_id,
        req.publish_at.map(DateTime::<Utc>::from),
    )
    .await?;

    Ok(Json(CreateResponse { id }))
}

async fn delete(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageJig>,
    path: web::Path<JigId>,
) -> Result<HttpResponse, <jig::Delete as ApiEndpoint>::Err> {
    db::jig::delete(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().into())
}

async fn update(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageJig>,
    req: Option<Json<<jig::Update as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<HttpResponse, <jig::Update as ApiEndpoint>::Err> {
    let req = req.map_or_else(Default::default, Json::into_inner);
    let exists = db::jig::update(
        &*db,
        path.into_inner(),
        req.display_name.as_deref(),
        req.author_id,
        req.cover.as_ref(),
        req.modules.as_deref(),
        req.ending.as_ref(),
        req.publish_at.map(|it| it.map(DateTime::<Utc>::from)),
    )
    .await?;

    if !exists {
        return Err(UpdateError::NotFound);
    }

    Ok(HttpResponse::NoContent().into())
}

async fn get(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Get as ApiEndpoint>::Res>, <jig::Get as ApiEndpoint>::Err> {
    let jig = db::jig::get(&db, path.into_inner())
        .await?
        .ok_or(GetError::NotFound)?;

    Ok(Json(GetResponse { jig }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(jig::Get::PATH, jig::Get::METHOD.route().to(get))
        .route(jig::Create::PATH, jig::Create::METHOD.route().to(create))
        .route(jig::Update::PATH, jig::Update::METHOD.route().to(update))
        .route(jig::Delete::PATH, jig::Delete::METHOD.route().to(delete));
}
