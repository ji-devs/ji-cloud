use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::module, ApiEndpoint, PathParts},
    domain::{
        asset::{AssetId, AssetType},
        module::{ModuleId, ModuleResponse},
        CreateResponse,
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
    let user_id = auth.user_id();

    let (id, _index) = match req.parent_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, user_id, Some(jig_id)).await?;

            db::jig::module::create(&*db, jig_id, req.body, is_complete).await?
        }
        AssetId::CourseId(course_id) => {
            db::course::authz(&*db, user_id, Some(course_id)).await?;
            db::course::module::create(&*db, course_id, req.body, is_complete).await?
        }
        AssetId::ResourceId(resource_id) => {
            db::resource::authz(&*db, user_id, Some(resource_id)).await?;
            db::resource::module::create(&*db, resource_id, req.body, is_complete).await?
        }
    };

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Get a Live.
async fn get_live(
    db: Data<PgPool>,
    path: web::Path<(AssetType, ModuleId)>,
) -> Result<Json<<module::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let asset = path.0;
    let module_id = path.1;

    let module = match asset {
        AssetType::Jig => db::jig::module::get_live(&db, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
        AssetType::Course => db::course::module::get_live(&db, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
        AssetType::Resource => db::resource::module::get_live(&db, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
    };

    Ok(Json(ModuleResponse { module }))
}

/// Get a Draft module
async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<(AssetType, ModuleId)>,
) -> Result<Json<<module::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let asset = path.0;
    let module_id = path.1;

    let module = match asset {
        AssetType::Jig => db::jig::module::get_draft(&db, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
        AssetType::Course => db::course::module::get_draft(&db, module_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?,
        AssetType::Resource => db::resource::module::get_draft(&db, module_id)
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
    let user_id = auth.user_id();

    let exists = match req.parent_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, user_id, Some(jig_id)).await?;

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
            db::course::authz(&*db, user_id, Some(course_id)).await?;

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
        AssetId::ResourceId(resource_id) => {
            db::resource::authz(&*db, user_id, Some(resource_id)).await?;

            db::resource::module::update(
                &*db,
                resource_id,
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
    let user_id = auth.user_id();

    match parent_id {
        AssetId::JigId(jig_id) => {
            db::jig::authz(&*db, user_id, Some(jig_id)).await?;

            db::jig::module::delete(&*db, jig_id, module_id).await?;
        }
        AssetId::CourseId(course_id) => {
            db::course::authz(&*db, user_id, Some(course_id)).await?;

            db::course::module::delete(&*db, course_id, module_id).await?;
        }
        AssetId::ResourceId(resource_id) => {
            db::resource::authz(&*db, user_id, Some(resource_id)).await?;

            db::resource::module::delete(&*db, resource_id, module_id).await?;
        }
    };

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <module::Create as ApiEndpoint>::Path::PATH,
        module::Create::METHOD.route().to(create),
    )
    .route(
        <module::GetLive as ApiEndpoint>::Path::PATH,
        module::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <module::GetDraft as ApiEndpoint>::Path::PATH,
        module::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <module::Update as ApiEndpoint>::Path::PATH,
        module::Update::METHOD.route().to(update),
    )
    .route(
        <module::Delete as ApiEndpoint>::Path::PATH,
        module::Delete::METHOD.route().to(delete),
    );
}
