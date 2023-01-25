use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::pro_dev, ApiEndpoint, PathParts},
    domain::{
        asset::DraftOrLive,
        pro_dev::{
            unit::{ProDevUnit, ProDevUnitId},
            ProDevId,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db::{self},
    error,
    extractor::TokenUser,
};

/// Create a new Pro Dev Unit.
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<ProDevId>,
    req: Json<<pro_dev::unit::Create as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<pro_dev::unit::Create as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let req = req.into_inner();
    let user_id = auth.user_id();
    let pro_dev_id = path.to_owned();

    db::pro_dev::authz(&*db, user_id, Some(pro_dev_id)).await?;

    let id = db::pro_dev::unit::create(
        &*db,
        pro_dev_id,
        req.display_name,
        req.description,
        req.value,
    )
    .await?;

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

/// Update a Pro Dev Unit.
async fn update(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<(ProDevId, ProDevUnitId)>,
    req: Json<<pro_dev::unit::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let (pro_dev_id, pro_dev_unit_id) = path.into_inner();
    let req = req.into_inner();
    let user_id = auth.user_id();

    db::pro_dev::authz(&*db, user_id, Some(pro_dev_id)).await?;

    db::pro_dev::unit::update(
        &*db,
        pro_dev_id,
        pro_dev_unit_id,
        req.display_name,
        req.description,
        req.value,
        req.index,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get a Pro Dev Unit on a draft Pro Dev.
async fn get_draft(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<(ProDevId, ProDevUnitId)>,
) -> Result<Json<<pro_dev::unit::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let (pro_dev_id, pro_dev_unit_id) = path.into_inner();

    let (id, display_name, description, unit_value) =
        db::pro_dev::unit::get(&db, pro_dev_id, DraftOrLive::Draft, pro_dev_unit_id).await?;

    Ok(Json(ProDevUnit {
        id,
        display_name,
        description,
        value: unit_value,
    }))
}

/// Get a Pro Dev Unit on a live Pro Dev.
async fn get_live(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<(ProDevId, ProDevUnitId)>,
) -> Result<Json<<pro_dev::unit::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let (pro_dev_id, pro_dev_unit_id) = path.into_inner();

    let (id, display_name, description, value) =
        db::pro_dev::unit::get(&db, pro_dev_id, DraftOrLive::Live, pro_dev_unit_id).await?;

    Ok(Json(ProDevUnit {
        id,
        display_name,
        description,
        value,
    }))
}

/// Delete a Pro Dev Unit.
async fn delete(
    db: Data<PgPool>,
    auth: TokenUser,
    path: Path<(ProDevId, ProDevUnitId)>,
) -> Result<HttpResponse, error::Delete> {
    let (pro_dev_id, pro_dev_unit_id) = path.into_inner();
    let user_id = auth.user_id();

    db::pro_dev::authz(&*db, user_id, Some(pro_dev_id)).await?;

    db::pro_dev::unit::delete(&*db, pro_dev_id, pro_dev_unit_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <pro_dev::unit::Create as ApiEndpoint>::Path::PATH,
        pro_dev::unit::Create::METHOD.route().to(create),
    )
    .route(
        <pro_dev::unit::Update as ApiEndpoint>::Path::PATH,
        pro_dev::unit::Update::METHOD.route().to(update),
    )
    .route(
        <pro_dev::unit::GetDraft as ApiEndpoint>::Path::PATH,
        pro_dev::unit::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <pro_dev::unit::GetLive as ApiEndpoint>::Path::PATH,
        pro_dev::unit::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <pro_dev::unit::Delete as ApiEndpoint>::Path::PATH,
        pro_dev::unit::Delete::METHOD.route().to(delete),
    );
}
