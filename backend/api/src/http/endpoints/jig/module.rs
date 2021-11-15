use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::jig::module, ApiEndpoint},
    domain::{
        jig::{
            module::{ModuleId, ModuleResponse, StableModuleId, StableOrUniqueId},
            JigId,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;

// use serde_qs::actix::QsQuery;

use crate::{db, error, extractor::TokenUser};

/// Create a new module on a draft JIG.
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    parent: Path<JigId>,
    req: Json<<module::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let parent_id = parent.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let req = req.into_inner();

    let (id, _index) = db::jig::module::create(&*db, parent_id, req.body).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Get a module from a live JIG.
async fn get_live(
    db: Data<PgPool>,
    path: web::Path<(JigId, String)>,
    query: Query<<module::GetLive as ApiEndpoint>::Req>,
) -> Result<Json<<module::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let path = path.into_inner();
    let query = query.into_inner();

    let q: &str = &query.q;

    let module_id = match q {
        "stable" => StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(&path.1)?)),

        "unique" => StableOrUniqueId::Unique(ModuleId(uuid::Uuid::parse_str(&path.1)?)),

        _ => {
            return Err(error::NotFound::ResourceNotFound);
        }
    };

    let module = db::jig::module::get_live(&db, path.0, module_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(ModuleResponse { module }))
}

/// Get a module from a draft JIG.
///
/// FIXME dedup this from live JIG
async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<(JigId, String)>,
    query: Query<<module::GetLive as ApiEndpoint>::Req>,
) -> Result<Json<<module::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let path = path.into_inner();
    let query = query.into_inner();

    let q: &str = &query.q;

    let module_id = match q {
        "stable" => StableOrUniqueId::Stable(StableModuleId(uuid::Uuid::parse_str(&path.1)?)),

        "unique" => StableOrUniqueId::Unique(ModuleId(uuid::Uuid::parse_str(&path.1)?)),

        _ => {
            return Err(error::NotFound::ResourceNotFound);
        }
    };

    let module = db::jig::module::get_draft(&db, path.0, module_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(ModuleResponse { module }))
}

/// Update a module in a draft JIG.
async fn update(
    db: Data<PgPool>,
    auth: TokenUser,
    path: web::Path<JigId>,
    req: Json<<module::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::NotFound> {
    let parent_id = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let req = req.into_inner();

    let exists = db::jig::module::update(
        &*db,
        parent_id,
        req.id,
        req.body.as_ref(),
        req.index,
        req.is_complete,
    )
    .await?;

    if !exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a module from a draft JIG.
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: web::Path<JigId>,
    req: Json<<module::Delete as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Delete> {
    let parent_id = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    db::jig::module::delete(&*db, parent_id, req.id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        module::Create::PATH,
        module::Create::METHOD.route().to(create),
    )
    .route(
        module::GetLive::PATH,
        module::GetLive::METHOD.route().to(get_live),
    )
    .route(
        module::GetDraft::PATH,
        module::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        module::Update::PATH,
        module::Update::METHOD.route().to(update),
    )
    .route(
        module::Delete::PATH,
        module::Delete::METHOD.route().to(delete),
    );
}
