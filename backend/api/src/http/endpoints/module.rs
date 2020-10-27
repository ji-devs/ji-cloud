use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::module, ApiEndpoint},
    domain::{
        jig::module::{CreateRequest, GetResponse, ModuleId},
        CreateResponse,
    },
    error::{GetError, UpdateError},
};
use sqlx::PgPool;

use crate::{
    db,
    extractor::{AuthUserWithScope, ScopeManageModule, WrapAuthClaimsNoDb},
};

async fn create(
    db: Data<PgPool>,
    _auth: AuthUserWithScope<ScopeManageModule>,
    req: Option<Json<<module::Create as ApiEndpoint>::Req>>,
) -> Result<Json<<module::Create as ApiEndpoint>::Res>, <module::Create as ApiEndpoint>::Err> {
    let req = req.map_or_else(CreateRequest::default, Json::into_inner);
    let id = db::module::create(&*db, req.kind, req.body.as_ref()).await?;

    Ok(Json(CreateResponse { id }))
}

async fn delete(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageModule>,
    path: web::Path<ModuleId>,
) -> Result<HttpResponse, <module::Delete as ApiEndpoint>::Err> {
    db::module::delete(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().into())
}

async fn update(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageModule>,
    req: Option<Json<<module::Update as ApiEndpoint>::Req>>,
    path: web::Path<ModuleId>,
) -> Result<HttpResponse, <module::Update as ApiEndpoint>::Err> {
    let req = req.map_or_else(Default::default, Json::into_inner);
    let exists = db::module::update(&*db, path.into_inner(), req.kind, req.body.as_ref()).await?;

    if !exists {
        return Err(UpdateError::NotFound);
    }

    Ok(HttpResponse::NoContent().into())
}

async fn get(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    path: web::Path<ModuleId>,
) -> Result<Json<<module::Get as ApiEndpoint>::Res>, <module::Get as ApiEndpoint>::Err> {
    let module = db::module::get(&db, path.into_inner())
        .await?
        .ok_or(GetError::NotFound)?;

    Ok(Json(GetResponse { module }))
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
