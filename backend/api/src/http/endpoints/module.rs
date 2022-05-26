use actix_web::{
    web::{self, Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::module, ApiEndpoint},
    domain::{
        module::{ModuleId, ModuleResponse},
        CreateResponse,
        asset::AssetId,
    },
};
use sqlx::PgPool;

// use serde_qs::actix::QsQuery;

use crate::{db, error, extractor::TokenUser};

/// Create a new Draft module
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Json<<module::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let req = req.into_inner();
    let is_complete = req.body.is_complete();

    let (id, _index) = match req.parent_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, auth.0.user_id, Some(jig_id)).await?;

            db::jig::module::create(&*db, jig_id, req.body, is_complete).await?
        }
        AssetId::CourseId(course_id) => {
            db::course::authz(&*db, auth.0.user_id, Some(course_id)).await?;
            db::course::module::create(&*db, course_id, req.body, is_complete).await?
        }
    };

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Get a Live.
async fn get_live(
    db: Data<PgPool>,
    path: web::Path<ModuleId>,
    query: Query<<module::GetLive as ApiEndpoint>::Req>,
) -> Result<Json<<module::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let (module_id, query) = (path.into_inner(), query.into_inner());

    let module = match query.parent_id {
        AssetId::JigId(jig_id) => db::jig::module::get_live(&db, jig_id, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
        AssetId::CourseId(course_id) => db::course::module::get_live(&db, course_id, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
    };

    Ok(Json(ModuleResponse { module }))
}

/// Get a Draft module
async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<ModuleId>,
    query: Query<<module::GetDraft as ApiEndpoint>::Req>,
) -> Result<Json<<module::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let (module_id, query) = (path.into_inner(), query.into_inner());

    let module = match query.parent_id {
        AssetId::JigId(jig_id) => db::jig::module::get_draft(&db, jig_id, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
        AssetId::CourseId(course_id) => db::course::module::get_draft(&db, course_id, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
    };

    Ok(Json(ModuleResponse { module }))
}

/// Update a Draft module.
async fn update(
    db: Data<PgPool>,
    auth: TokenUser,
    path: web::Path<ModuleId>,
    req: Json<<module::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::NotFound> {
    let (req, module_id) = (req.into_inner(), path.into_inner());

    let exists = match req.parent_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, auth.0.user_id, Some(jig_id)).await?;

            db::jig::module::update(
                &*db,
                jig_id,
                module_id,
                req.body.as_ref(),
                req.index,
                req.is_complete,
            )
            .await?
        }
        AssetId::CourseId(course_id) => {
            db::course::authz(&*db, auth.0.user_id, Some(course_id)).await?;

            db::course::module::update(
                &*db,
                course_id,
                module_id,
                req.body.as_ref(),
                req.index,
                req.is_complete,
            )
            .await?
        }
    };

    if !exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a Draft module.
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: web::Path<ModuleId>,
    req: Json<<module::Delete as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Delete> {
    let (module_id, parent_id) = (path.into_inner(), req.parent_id);

    match parent_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, auth.0.user_id, Some(jig_id)).await?;

            db::jig::module::delete(&*db, jig_id, module_id).await?;
        }
        AssetId::CourseId(course_id) => {
            db::course::authz(&*db, auth.0.user_id, Some(course_id)).await?;

            db::course::module::delete(&*db, course_id, module_id).await?;
        }
    };

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
