use crate::{db, extractor::WrapAuthClaimsNoDb};
use actix_web::{
    http,
    web::{Data, Json, Path, ServiceConfig},
};
use shared::{
    api::{endpoints::image, ApiEndpoint},
    domain::image::{meta::MetaKind, CreateResponse, GetResponse, ImageId},
    error::image::{CreateError, GetError},
};
use sqlx::{postgres::PgDatabaseError, PgPool};
use url::Url;
use uuid::Uuid;

mod meta;

// attempts to grab a uuid out of a string in the shape:
// Key (<key>)=(<uuid>)<postfix>
fn extract_uuid(s: &str) -> Option<Uuid> {
    // <uuid>)<postfix)
    let s = dbg!(s.split("(").nth(2)?);
    let s = dbg!(&s[0..s.find(")")?]);
    s.parse().ok()
}

fn handle_metadata_err(err: sqlx::Error) -> CreateError {
    let db_err = match &err {
        sqlx::Error::Database(e) => e.downcast_ref::<PgDatabaseError>(),
        _ => return err.into(),
    };

    let id = db_err.detail().and_then(extract_uuid);

    match dbg!(db_err.constraint()) {
        Some("image_affiliation_affiliation_id_fkey") => CreateError::MissingMetadata {
            id,
            kind: MetaKind::Affiliation,
        },

        Some("image_age_range_age_range_id_fkey") => CreateError::MissingMetadata {
            id,
            kind: MetaKind::AgeRange,
        },

        Some("image_style_style_id_fkey") => CreateError::MissingMetadata {
            id,
            kind: MetaKind::Style,
        },

        Some("image_category_category_id_fkey") => CreateError::MissingCategory(id),

        _ => return err.into(),
    }
}

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
        &req.categories,
    )
    .await
    .map_err(handle_metadata_err)?;

    txn.commit().await?;

    Ok((
        Json(CreateResponse {
            id,
            upload_url: presigned_url,
        }),
        http::StatusCode::CREATED,
    ))
}

pub async fn get(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Path<ImageId>,
) -> Result<Json<<image::Get as ApiEndpoint>::Res>, <image::Get as ApiEndpoint>::Err> {
    db::image::get(&db, req.into_inner())
        .await?
        .map(|image| Json(GetResponse { image }))
        .ok_or(GetError::NotFound)
}

pub fn configure(cfg: &mut ServiceConfig) {
    meta::configure(cfg);
    cfg.route(
        image::Create::PATH,
        image::Create::METHOD.route().to(create),
    )
    .route(image::Get::PATH, image::Get::METHOD.route().to(get));
}
