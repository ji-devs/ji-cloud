use actix_web::HttpResponse;
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
    error::{
        jig::{CreateError, CreateErrorExt, UpdateError, UpdateErrorExt},
        GetError,
    },
};
use sqlx::PgPool;

use crate::{
    db::{self, meta::MetaWrapperError},
    extractor::{AuthUserWithScope, ScopeManageJig, WrapAuthClaimsNoDb},
};

impl From<MetaWrapperError> for CreateError {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => CreateError::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                CreateError::Extra(CreateErrorExt::NonExistantMetadata { id, kind })
            }
        }
    }
}

impl From<MetaWrapperError> for UpdateError {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => UpdateError::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                UpdateError::Extra(UpdateErrorExt::NonExistantMetadata { id, kind })
            }
        }
    }
}

/// Create a jig.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    auth: AuthUserWithScope<ScopeManageJig>,
    req: Option<Json<<jig::Create as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Create as ApiEndpoint>::Res>, <jig::Create as ApiEndpoint>::Err> {
    let req = req.map_or_else(JigCreateRequest::default, Json::into_inner);
    let creator_id = auth.claims.id;

    let id = db::jig::create(
        &*db,
        req.display_name.as_deref(),
        req.cover,
        &req.modules,
        &req.content_types,
        req.ending,
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
    _claims: AuthUserWithScope<ScopeManageJig>,
    path: web::Path<JigId>,
) -> Result<HttpResponse, <jig::Delete as ApiEndpoint>::Err> {
    db::jig::delete(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().into())
}

/// Update a jig.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageJig>,
    req: Option<Json<<jig::Update as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<NoContent, <jig::Update as ApiEndpoint>::Err> {
    let req = req.map_or_else(Default::default, Json::into_inner);
    let exists = db::jig::update(
        &*db,
        path.into_inner(),
        req.display_name.as_deref(),
        req.author_id,
        req.cover,
        req.modules.as_deref(),
        req.ending,
        req.content_types.as_deref(),
        req.publish_at.map(|it| it.map(DateTime::<Utc>::from)),
    )
    .await
    .map_err(db::meta::handle_metadata_err)?;

    if !exists {
        return Err(UpdateError::NotFound);
    }

    Ok(NoContent)
}

/// Get a jig.
#[api_v2_operation]
async fn get(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Get as ApiEndpoint>::Res>, <jig::Get as ApiEndpoint>::Err> {
    let jig = db::jig::get(&db, path.into_inner())
        .await?
        .ok_or(GetError::NotFound)?;

    Ok(Json(JigResponse { jig }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(jig::Get::PATH, jig::Get::METHOD.route().to(get))
        .route(jig::Create::PATH, jig::Create::METHOD.route().to(create))
        .route(jig::Update::PATH, jig::Update::METHOD.route().to(update))
        .route(jig::Delete::PATH, jig::Delete::METHOD.route().to(delete));
}
