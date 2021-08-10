//! Manages image tags
use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUserWithScope},
};
use paperclip::actix::web::Path;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json},
    CreatedJson, NoContent,
};
use shared::domain::image::tag::ImageTagResponse;
use shared::domain::meta::ImageTagIndex;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::tag::ImageTagListResponse,
};
use sqlx::PgPool;

#[api_v2_operation]
pub(super) async fn list(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
) -> Result<Json<<endpoints::image::tag::List as ApiEndpoint>::Res>, error::NotFound> {
    let image_tags = db::image::tag::list(db.as_ref()).await?;

    Ok(Json(ImageTagListResponse { image_tags }))
}

#[api_v2_operation]
pub(super) async fn create(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
    index: Path<ImageTagIndex>,
    req: Json<<endpoints::image::tag::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<endpoints::image::tag::Create as ApiEndpoint>::Res>, error::Tag> {
    let res =
        db::image::tag::create(db.as_ref(), index.into_inner(), req.display_name.as_str()).await?;

    Ok(CreatedJson(ImageTagResponse {
        index: res.0,
        display_name: res.1,
    }))
}

#[api_v2_operation]
pub(super) async fn update(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
    index: Path<i16>,
    req: Json<<endpoints::image::tag::Update as ApiEndpoint>::Req>,
) -> Result<NoContent, error::Tag> {
    let req = req.into_inner();

    let _resp = db::image::tag::update(
        db.as_ref(),
        index.into_inner(),
        req.display_name.as_deref(),
        req.index.map(|it| it.0),
    )
    .await?;

    Ok(NoContent)
}

#[api_v2_operation]
pub(super) async fn delete(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeAdmin>,
    req: Path<i16>,
) -> Result<NoContent, error::Tag> {
    db::image::tag::delete(db.as_ref(), req.into_inner()).await?;

    Ok(NoContent)
}
