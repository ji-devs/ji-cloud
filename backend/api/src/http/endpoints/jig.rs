use chrono::{DateTime, Utc};
use paperclip::actix::{
    api_v2_operation,
    web::{self, Data, Json, ServiceConfig},
    NoContent,
};
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::{
        jig::{JigCreateRequest, JigId, JigResponse},
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db,
    error::{self, UpdateWithMetadata},
    extractor::{ScopeManageJig, TokenUser, TokenUserWithScope},
};

/// Create a jig.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    auth: TokenUserWithScope<ScopeManageJig>,
    req: Option<Json<<jig::Create as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Create as ApiEndpoint>::Res>, error::CreateWithMetadata> {
    let req = req.map_or_else(JigCreateRequest::default, Json::into_inner);
    let creator_id = auth.claims.sub;

    let id = db::jig::create(
        &*db,
        req.display_name.as_deref(),
        &req.modules,
        &req.content_types,
        creator_id,
        req.publish_at.map(DateTime::<Utc>::from),
    )
    .await
    .map_err(db::meta::handle_metadata_err)?;

    Ok(Json(CreateResponse { id }))
}

/// Delete a jig.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageJig>,
    path: web::Path<JigId>,
) -> Result<NoContent, error::Delete> {
    db::jig::delete(&*db, path.into_inner()).await?;

    Ok(NoContent)
}

/// Update a jig.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageJig>,
    req: Option<Json<<jig::Update as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<NoContent, UpdateWithMetadata> {
    let req = req.map_or_else(Default::default, Json::into_inner);
    let exists = db::jig::update(
        &*db,
        path.into_inner(),
        req.display_name.as_deref(),
        req.author_id,
        req.modules.as_deref(),
        req.content_types.as_deref(),
        req.publish_at.map(|it| it.map(DateTime::<Utc>::from)),
    )
    .await
    .map_err(db::meta::handle_metadata_err)?;

    if !exists {
        return Err(UpdateWithMetadata::ResourceNotFound);
    }

    Ok(NoContent)
}

/// Get a jig.
#[api_v2_operation]
async fn get(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Get as ApiEndpoint>::Res>, error::NotFound> {
    let jig = db::jig::get(&db, path.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(JigResponse { jig }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(jig::Get::PATH, jig::Get::METHOD.route().to(get))
        .route(jig::Create::PATH, jig::Create::METHOD.route().to(create))
        .route(jig::Update::PATH, jig::Update::METHOD.route().to(update))
        .route(jig::Delete::PATH, jig::Delete::METHOD.route().to(delete));
}
