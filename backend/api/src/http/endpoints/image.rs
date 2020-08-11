use crate::{db, extractor::WrapAuthClaimsNoDb};
use actix_web::{
    http,
    web::{Data, Json, ServiceConfig},
};
use shared::{
    api::{endpoints::image, ApiEndpoint},
    domain::image::CreateResponse,
};
use sqlx::PgPool;
use url::Url;

mod meta;

pub async fn create(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Json<<image::Create as ApiEndpoint>::Req>,
) -> Result<
    (Json<<image::Create as ApiEndpoint>::Res>, http::StatusCode),
    <image::Create as ApiEndpoint>::Err,
> {
    let req = req.into_inner();

    // TODO: actually get this from aws, this is just a fake url to make stuff work
    let presigned_url: Url = "https://aws.wubwub".parse()?;

    let mut txn = db.begin().await?;
    let id = db::image::create(
        &mut txn,
        presigned_url.as_str(),
        &req.name,
        &req.description,
        req.is_premium,
        req.publish_at.as_ref(),
    )
    .await?;

    // todo: don't 500 when one of the ids doesn't exist.
    db::image::add_metadata(
        &mut txn,
        id,
        &req.affiliations,
        &req.age_ranges,
        &req.styles,
    )
    .await?;

    Ok((
        Json(CreateResponse {
            id,
            upload_url: presigned_url,
        }),
        http::StatusCode::CREATED,
    ))
}

pub fn configure(cfg: &mut ServiceConfig) {
    meta::configure(cfg);
    cfg.route(
        image::Create::PATH,
        image::Create::METHOD.route().to(create),
    );
}
