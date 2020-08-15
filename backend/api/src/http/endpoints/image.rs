use crate::{db, extractor::WrapAuthClaimsNoDb, s3::S3Client};
use actix_web::{
    http,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use db::image::nul_if_empty;
use futures::TryStreamExt;
use http::StatusCode;
use shared::{
    api::{endpoints::image, ApiEndpoint},
    domain::image::{
        meta::MetaKind, CreateResponse, GetOneResponse, GetResponse, Image, ImageId, UpdateRequest,
    },
    error::image::{CreateError, DeleteError, GetError, GetOneError, UpdateError},
};
use sqlx::{postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

mod meta;

// attempts to grab a uuid out of a string in the shape:
// Key (<key>)=(<uuid>)<postfix>
fn extract_uuid(s: &str) -> Option<Uuid> {
    // <uuid>)<postfix)
    let s = s.split("(").nth(2)?;
    let s = &s[0..s.find(")")?];
    s.parse().ok()
}

enum MetaWrapperError {
    Sqlx(sqlx::Error),
    MissingMetadata { id: Option<Uuid>, kind: MetaKind },
}

impl From<MetaWrapperError> for CreateError {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => CreateError::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                CreateError::MissingMetadata { id, kind }
            }
        }
    }
}

impl From<MetaWrapperError> for UpdateError {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => UpdateError::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                UpdateError::MissingMetadata { id, kind }
            }
        }
    }
}

fn handle_metadata_err(err: sqlx::Error) -> MetaWrapperError {
    let db_err = match &err {
        sqlx::Error::Database(e) => e.downcast_ref::<PgDatabaseError>(),
        _ => return MetaWrapperError::Sqlx(err),
    };

    let id = db_err.detail().and_then(extract_uuid);

    match db_err.constraint() {
        Some("image_affiliation_affiliation_id_fkey") => MetaWrapperError::MissingMetadata {
            id,
            kind: MetaKind::Affiliation,
        },

        Some("image_age_range_age_range_id_fkey") => MetaWrapperError::MissingMetadata {
            id,
            kind: MetaKind::AgeRange,
        },

        Some("image_style_style_id_fkey") => MetaWrapperError::MissingMetadata {
            id,
            kind: MetaKind::Style,
        },

        Some("image_category_category_id_fkey") => MetaWrapperError::MissingMetadata {
            id,
            kind: MetaKind::Category,
        },

        _ => MetaWrapperError::Sqlx(err),
    }
}

async fn create(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    // _claims: WrapAuthClaimsNoDb,
    req: Json<<image::Create as ApiEndpoint>::Req>,
) -> Result<
    (Json<<image::Create as ApiEndpoint>::Res>, http::StatusCode),
    <image::Create as ApiEndpoint>::Err,
> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;
    let id = db::image::create(
        &mut txn,
        &req.name,
        &req.description,
        req.is_premium,
        req.publish_at.as_ref(),
    )
    .await?;

    db::image::update_metadata(
        &mut txn,
        id,
        nul_if_empty(&req.affiliations),
        nul_if_empty(&req.age_ranges),
        nul_if_empty(&req.styles),
        nul_if_empty(&req.categories),
    )
    .await
    .map_err(handle_metadata_err)?;

    txn.commit().await?;

    Ok((
        Json(CreateResponse {
            id,
            upload_url: s3.presigned_image_put_url(id)?,
        }),
        http::StatusCode::CREATED,
    ))
}

async fn get_one(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    _claims: WrapAuthClaimsNoDb,
    req: Path<ImageId>,
) -> Result<Json<<image::GetOne as ApiEndpoint>::Res>, <image::GetOne as ApiEndpoint>::Err> {
    let metadata = db::image::get_one(&db, req.into_inner())
        .await?
        .ok_or(GetOneError::NotFound)?;

    let id = metadata.id;

    Ok(Json(GetOneResponse {
        metadata,
        url: s3.presigned_image_get_url(id)?,
    }))
}

// todo: use algolia
async fn get(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    _claims: WrapAuthClaimsNoDb,
    query: Option<Query<<image::Get as ApiEndpoint>::Req>>,
) -> Result<Json<<image::Get as ApiEndpoint>::Res>, <image::Get as ApiEndpoint>::Err> {
    let _query = query.map_or_else(Default::default, Query::into_inner);

    let images: Vec<_> = db::image::get(db.as_ref())
        .err_into::<GetError>()
        .and_then(|metadata: Image| async {
            Ok(GetOneResponse {
                url: s3.presigned_image_get_url(metadata.id)?,
                metadata,
            })
        })
        .try_collect()
        .await?;

    Ok(Json(GetResponse { images }))
}

async fn update(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Option<Json<<image::UpdateMetadata as ApiEndpoint>::Req>>,
    id: Path<ImageId>,
) -> Result<HttpResponse, <image::UpdateMetadata as ApiEndpoint>::Err> {
    let req = req.map_or_else(UpdateRequest::default, Json::into_inner);
    let id = id.into_inner();
    let mut txn = db.begin().await?;

    let exists = db::image::update(
        &mut txn,
        id,
        req.name,
        req.description,
        req.is_premium,
        req.publish_at,
    )
    .await?;

    if !exists {
        return Err(UpdateError::NotFound);
    }

    db::image::update_metadata(
        &mut txn,
        id,
        req.affiliations.as_deref(),
        req.age_ranges.as_deref(),
        req.styles.as_deref(),
        req.categories.as_deref(),
    )
    .await
    .map_err(handle_metadata_err)?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}

fn check_conflict_image_delete(err: sqlx::Error) -> DeleteError {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            DeleteError::Conflict
        }
        _ => DeleteError::InternalServerError(err.into()),
    }
}

async fn delete(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Path<ImageId>,
    s3: Data<S3Client>,
) -> Result<HttpResponse, <image::Delete as ApiEndpoint>::Err> {
    let image = req.into_inner();
    db::image::delete(&db, image)
        .await
        .map_err(check_conflict_image_delete)?;

    s3.delete_image(image).await?;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}

pub fn configure(cfg: &mut ServiceConfig) {
    meta::configure(cfg);
    cfg.route(
        image::Create::PATH,
        image::Create::METHOD.route().to(create),
    )
    .route(
        image::GetOne::PATH,
        image::GetOne::METHOD.route().to(get_one),
    )
    .route(image::Get::PATH, image::Get::METHOD.route().to(get))
    .route(
        image::UpdateMetadata::PATH,
        image::UpdateMetadata::METHOD.route().to(update),
    )
    .route(
        image::Delete::PATH,
        image::Delete::METHOD.route().to(delete),
    );
}
