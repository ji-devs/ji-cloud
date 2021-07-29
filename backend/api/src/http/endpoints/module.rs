use paperclip::actix::{
    api_v2_operation,
    web::{self, Data, Json, Path, ServiceConfig},
    NoContent,
};
use shared::{
    api::{endpoints::jig::module, ApiEndpoint},
    domain::{
        jig::{
            module::{ModuleId, ModuleIdOrIndex, ModuleResponse},
            JigId,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};

/// Create a new module.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    parent: Path<JigId>,
    req: Json<<module::Create as ApiEndpoint>::Req>,
) -> Result<Json<<module::Create as ApiEndpoint>::Res>, error::Auth> {
    let parent_id = parent.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let req = req.into_inner();
    let (id, _index) = db::module::create(&*db, parent_id, req.body).await?;

    Ok(Json(CreateResponse { id }))
}

/// Delete a module.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: web::Path<(JigId, ModuleId)>,
) -> Result<NoContent, error::Delete> {
    let (parent_id, module) = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    db::module::delete(&*db, parent_id, ModuleIdOrIndex::Id(module)).await?;

    Ok(NoContent)
}

/// Update a module.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<module::Update as ApiEndpoint>::Req>>,
    path: web::Path<(JigId, ModuleId)>,
) -> Result<NoContent, error::NotFound> {
    let (parent_id, module) = path.into_inner();

    db::jig::authz(&*db, auth.0.user_id, Some(parent_id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);
    let exists = db::module::update(
        &*db,
        parent_id,
        ModuleIdOrIndex::Id(module),
        req.body.as_ref(),
        req.index,
        req.is_complete,
    )
    .await?;

    if !exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    Ok(NoContent)
}

/// Get a module.
#[api_v2_operation]
async fn get(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<(JigId, ModuleId)>,
) -> Result<Json<<module::Get as ApiEndpoint>::Res>, error::NotFound> {
    let (parent_id, module) = path.into_inner();

    let module = db::module::get(&db, parent_id, ModuleIdOrIndex::Id(module))
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(ModuleResponse { module }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
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
