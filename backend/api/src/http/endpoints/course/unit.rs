use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::{endpoints::course, ApiEndpoint, PathParts},
    domain::{
        asset::DraftOrLive,
        course::{
            unit::{CourseUnit, CourseUnitId},
            CourseId,
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
    path: Path<CourseId>,
    req: Json<<course::unit::Create as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<course::unit::Create as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    error::Auth,
> {
    let req = req.into_inner();
    let user_id = auth.user_id();
    let course_id = path.to_owned();

    db::course::authz(&*db, user_id, Some(course_id)).await?;

    let id = db::course::unit::create(
        &*db,
        course_id,
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
    path: Path<(CourseId, CourseUnitId)>,
    req: Json<<course::unit::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Auth> {
    let (course_id, course_unit_id) = path.into_inner();
    let req = req.into_inner();
    let user_id = auth.user_id();

    db::course::authz(&*db, user_id, Some(course_id)).await?;

    db::course::unit::update(
        &*db,
        course_id,
        course_unit_id,
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
    path: Path<(CourseId, CourseUnitId)>,
) -> Result<Json<<course::unit::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let (course_id, course_unit_id) = path.into_inner();

    let (id, display_name, description, value) =
        db::course::unit::get(&db, course_id, DraftOrLive::Draft, course_unit_id).await?;

    Ok(Json(CourseUnit {
        id,
        display_name,
        description,
        value,
    }))
}

/// Get a Pro Dev Unit on a live Pro Dev.
async fn get_live(
    db: Data<PgPool>,
    _auth: TokenUser,
    path: Path<(CourseId, CourseUnitId)>,
) -> Result<Json<<course::unit::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let (course_id, course_unit_id) = path.into_inner();

    let (id, display_name, description, value) =
        db::course::unit::get(&db, course_id, DraftOrLive::Live, course_unit_id).await?;

    Ok(Json(CourseUnit {
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
    path: Path<(CourseId, CourseUnitId)>,
) -> Result<HttpResponse, error::Delete> {
    let (course_id, course_unit_id) = path.into_inner();
    let user_id = auth.user_id();

    db::course::authz(&*db, user_id, Some(course_id)).await?;

    db::course::unit::delete(&*db, course_id, course_unit_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <course::unit::Create as ApiEndpoint>::Path::PATH,
        course::unit::Create::METHOD.route().to(create),
    )
    .route(
        <course::unit::Update as ApiEndpoint>::Path::PATH,
        course::unit::Update::METHOD.route().to(update),
    )
    .route(
        <course::unit::GetDraft as ApiEndpoint>::Path::PATH,
        course::unit::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <course::unit::GetLive as ApiEndpoint>::Path::PATH,
        course::unit::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <course::unit::Delete as ApiEndpoint>::Path::PATH,
        course::unit::Delete::METHOD.route().to(delete),
    );
}
