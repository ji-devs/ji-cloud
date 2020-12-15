use paperclip::actix::{
    api_v2_operation,
    web::{self, Data, Json, ServiceConfig},
    NoContent,
};
use shared::{
    api::{endpoints::module, ApiEndpoint},
    domain::{
        jig::module::{ModuleCreateRequest, ModuleId, ModuleResponse},
        CreateResponse,
    },
    error::{GetError, UpdateError},
};
use sqlx::PgPool;

use crate::{
    db,
    extractor::{AuthUserWithScope, ScopeManageModule, WrapAuthClaimsNoDb},
};

/// Create a new module.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    _auth: AuthUserWithScope<ScopeManageModule>,
    req: Option<Json<<module::Create as ApiEndpoint>::Req>>,
) -> Result<Json<<module::Create as ApiEndpoint>::Res>, <module::Create as ApiEndpoint>::Err> {
    let req = req.map_or_else(ModuleCreateRequest::default, Json::into_inner);
    let id = db::module::create(&*db, req.kind, req.body.as_ref()).await?;

    Ok(Json(CreateResponse { id }))
}

/// Delete a module.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageModule>,
    path: web::Path<ModuleId>,
) -> Result<NoContent, <module::Delete as ApiEndpoint>::Err> {
    db::module::delete(&*db, path.into_inner()).await?;

    Ok(NoContent)
}

/// Update a module.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageModule>,
    req: Option<Json<<module::Update as ApiEndpoint>::Req>>,
    path: web::Path<ModuleId>,
) -> Result<NoContent, <module::Update as ApiEndpoint>::Err> {
    let req = req.map_or_else(Default::default, Json::into_inner);
    let exists = db::module::update(&*db, path.into_inner(), req.kind, req.body.as_ref()).await?;

    if !exists {
        return Err(UpdateError::NotFound);
    }

    Ok(NoContent)
}

/// Get a module.
#[api_v2_operation]
async fn get(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    path: web::Path<ModuleId>,
) -> Result<Json<<module::Get as ApiEndpoint>::Res>, <module::Get as ApiEndpoint>::Err> {
    let module = db::module::get(&db, path.into_inner())
        .await?
        .ok_or(GetError::NotFound)?;

    Ok(Json(ModuleResponse { module }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(module::Get::PATH, module::Get::METHOD.route().to(get))
        .route(
            module::Create::PATH,
            module::Create::METHOD.route().to(create),
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
