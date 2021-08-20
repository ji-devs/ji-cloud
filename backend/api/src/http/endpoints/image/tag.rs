//! Manages image tags
use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUserWithScope},
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use shared::domain::image::tag::ImageTagResponse;
use shared::domain::meta::ImageTagIndex;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::tag::ImageTagListResponse,
};
use sqlx::PgPool;

pub(super) async fn list(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
) -> Result<Json<<endpoints::image::tag::List as ApiEndpoint>::Res>, error::NotFound> {
    let image_tags = db::image::tag::list(db.as_ref()).await?;

    Ok(Json(ImageTagListResponse { image_tags }))
}

pub(super) async fn create(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
    index: Path<ImageTagIndex>,
    req: Json<<endpoints::image::tag::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Tag> {
    let res =
        db::image::tag::create(db.as_ref(), index.into_inner(), req.display_name.as_str()).await?;

    Ok(HttpResponse::Created().json(ImageTagResponse {
        index: res.0,
        display_name: res.1,
    }))
}

pub(super) async fn update(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
    index: Path<i16>,
    req: Json<<endpoints::image::tag::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Tag> {
    let req = req.into_inner();

    db::image::tag::update(
        db.as_ref(),
        index.into_inner(),
        req.display_name.as_deref(),
        req.index.map(|it| it.0),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub(super) async fn delete(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
    req: Path<i16>,
) -> Result<HttpResponse, error::Tag> {
    db::image::tag::delete(db.as_ref(), req.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}
