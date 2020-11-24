use crate::{
    algolia::AlgoliaClient,
    db::{self, nul_if_empty},
    extractor::{AuthUserWithScope, ScopeManageImage, WrapAuthClaimsNoDb},
    image_ops::generate_images,
    s3::{S3Client, S3LibraryKind, S3MediaVariant},
};
use actix_web::{
    http::{self, StatusCode},
    web::{self, Bytes, Data, Json, Path, PayloadConfig, Query, ServiceConfig},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        image::{CreateResponse, GetResponse, Image, ImageId, SearchResponse, UpdateRequest},
        meta::MetaKind,
    },
    error::{
        image::{CreateError, SearchError, UpdateError, UploadError},
        DeleteError, GetError,
    },
};
use sqlx::{postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

pub mod user {
    use crate::{
        db,
        extractor::WrapAuthClaimsNoDb,
        image_ops::generate_images,
        s3::{S3Client, S3LibraryKind, S3MediaVariant},
    };
    use actix_web::{
        http,
        web::{Bytes, Data, Json, Path},
        HttpResponse,
    };
    use futures::TryStreamExt;
    use shared::{
        api::{endpoints, ApiEndpoint},
        domain::{
            image::{
                user::{GetResponse, ListResponse, UserImage},
                ImageId, ImageKind,
            },
            CreateResponse,
        },
        error::{image::UploadError, GetError},
    };
    use sqlx::PgPool;

    pub(super) async fn create(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
    ) -> Result<
        (
            Json<<endpoints::image::user::Create as ApiEndpoint>::Res>,
            http::StatusCode,
        ),
        <endpoints::image::user::Create as ApiEndpoint>::Err,
    > {
        let id = db::image::user::create(db.as_ref()).await?;
        Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
    }

    pub(super) async fn upload(
        db: Data<PgPool>,
        s3: Data<S3Client>,
        _claims: WrapAuthClaimsNoDb,
        Path(id): Path<ImageId>,
        bytes: Bytes,
    ) -> Result<HttpResponse, <endpoints::image::Upload as ApiEndpoint>::Err> {
        if !db::image::user::exists(db.as_ref(), id).await? {
            return Err(shared::error::image::UploadError::NotFound);
        }

        let kind = ImageKind::Sticker;

        let res: Result<_, UploadError> = tokio::task::spawn_blocking(move || {
            let original =
                image::load_from_memory(&bytes).map_err(|_| UploadError::InvalidImage)?;
            Ok(generate_images(original, kind)?)
        })
        .await?;

        let (original, resized, thumbnail) = res?;
        s3.upload_images(S3LibraryKind::User, id, original, resized, thumbnail)
            .await?;

        Ok(HttpResponse::NoContent().into())
    }

    pub(super) async fn delete(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
        req: Path<ImageId>,
        s3: Data<S3Client>,
    ) -> Result<HttpResponse, <endpoints::image::Delete as ApiEndpoint>::Err> {
        let image = req.into_inner();
        db::image::user::delete(&db, image)
            .await
            .map_err(super::check_conflict_delete)?;

        let delete_image = |kind| s3.delete_image(S3LibraryKind::Global, kind, image);
        let ((), (), ()) = futures::future::join3(
            delete_image(S3MediaVariant::Original),
            delete_image(S3MediaVariant::Resized),
            delete_image(S3MediaVariant::Thumbnail),
        )
        .await;

        Ok(HttpResponse::new(http::StatusCode::NO_CONTENT))
    }

    pub(super) async fn get(
        db: Data<PgPool>,
        s3: Data<S3Client>,
        _claims: WrapAuthClaimsNoDb,
        req: Path<ImageId>,
    ) -> Result<
        Json<<endpoints::image::user::Get as ApiEndpoint>::Res>,
        <endpoints::image::user::Get as ApiEndpoint>::Err,
    > {
        let metadata = db::image::user::get(&db, req.into_inner())
            .await?
            .ok_or(GetError::NotFound)?;

        let id = metadata.id;

        Ok(Json(GetResponse {
            metadata,
            url: s3.image_presigned_get_url(S3LibraryKind::Global, S3MediaVariant::Resized, id)?,
            thumbnail_url: s3.image_presigned_get_url(
                S3LibraryKind::Global,
                S3MediaVariant::Thumbnail,
                id,
            )?,
        }))
    }

    pub(super) async fn list(
        db: Data<PgPool>,
        s3: Data<S3Client>,
        _claims: WrapAuthClaimsNoDb,
    ) -> Result<
        Json<<endpoints::image::user::List as ApiEndpoint>::Res>,
        <endpoints::image::user::List as ApiEndpoint>::Err,
    > {
        let images: Vec<_> = db::image::user::list(db.as_ref())
            .err_into::<GetError>()
            .and_then(|metadata: UserImage| async {
                Ok(GetResponse {
                    url: s3.image_presigned_get_url(
                        S3LibraryKind::Global,
                        S3MediaVariant::Resized,
                        metadata.id,
                    )?,
                    thumbnail_url: s3.image_presigned_get_url(
                        S3LibraryKind::Global,
                        S3MediaVariant::Thumbnail,
                        metadata.id,
                    )?,
                    metadata,
                })
            })
            .try_collect()
            .await?;

        Ok(Json(ListResponse { images }))
    }
}

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
                CreateError::NonExistantMetadata { id, kind }
            }
        }
    }
}

impl From<MetaWrapperError> for UpdateError {
    fn from(e: MetaWrapperError) -> Self {
        match e {
            MetaWrapperError::Sqlx(e) => UpdateError::InternalServerError(e.into()),
            MetaWrapperError::MissingMetadata { id, kind } => {
                UpdateError::NonExistantMetadata { id, kind }
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
    // _claims: AuthUserWithScope<ScopeManageImage>,
    req: Json<<endpoints::image::Create as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<endpoints::image::Create as ApiEndpoint>::Res>,
        http::StatusCode,
    ),
    <endpoints::image::Create as ApiEndpoint>::Err,
> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;
    let id = db::image::create(
        &mut txn,
        &req.name,
        &req.description,
        req.is_premium,
        req.publish_at.map(DateTime::<Utc>::from),
        req.kind,
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

    Ok((Json(CreateResponse { id }), http::StatusCode::CREATED))
}

async fn upload(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    Path(id): Path<ImageId>,
    bytes: Bytes,
) -> Result<HttpResponse, <endpoints::image::Upload as ApiEndpoint>::Err> {
    let kind = db::image::get_image_kind(db.as_ref(), id)
        .await?
        .ok_or(UploadError::NotFound)?;

    let res: Result<_, UploadError> = tokio::task::spawn_blocking(move || {
        let original = image::load_from_memory(&bytes).map_err(|_| UploadError::InvalidImage)?;
        Ok(generate_images(original, kind)?)
    })
    .await?;

    let (original, resized, thumbnail) = res?;
    s3.upload_images(S3LibraryKind::Global, id, original, resized, thumbnail)
        .await?;

    Ok(HttpResponse::NoContent().into())
}

async fn get_one(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    _claims: WrapAuthClaimsNoDb,
    req: Path<ImageId>,
) -> Result<
    Json<<endpoints::image::Get as ApiEndpoint>::Res>,
    <endpoints::image::Get as ApiEndpoint>::Err,
> {
    let metadata = db::image::get_one(&db, req.into_inner())
        .await?
        .ok_or(GetError::NotFound)?;

    let id = metadata.id;

    Ok(Json(GetResponse {
        metadata,
        url: s3.image_presigned_get_url(S3LibraryKind::Global, S3MediaVariant::Resized, id)?,
        thumbnail_url: s3.image_presigned_get_url(
            S3LibraryKind::Global,
            S3MediaVariant::Thumbnail,
            id,
        )?,
    }))
}

async fn get(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    algolia: Data<AlgoliaClient>,
    _claims: WrapAuthClaimsNoDb,
    query: Option<Query<<endpoints::image::Search as ApiEndpoint>::Req>>,
) -> Result<
    Json<<endpoints::image::Search as ApiEndpoint>::Res>,
    <endpoints::image::Search as ApiEndpoint>::Err,
> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (ids, pages) = algolia
        .search_image(
            &query.q,
            query.page,
            query.is_premium,
            query.is_published,
            &query.styles,
            &query.age_ranges,
            &query.affiliations,
            &query.categories,
        )
        .await?;

    let images: Vec<_> = db::image::get(db.as_ref(), &ids)
        .err_into::<SearchError>()
        .and_then(|metadata: Image| async {
            Ok(GetResponse {
                url: s3.image_presigned_get_url(
                    S3LibraryKind::Global,
                    S3MediaVariant::Resized,
                    metadata.id,
                )?,
                thumbnail_url: s3.image_presigned_get_url(
                    S3LibraryKind::Global,
                    S3MediaVariant::Thumbnail,
                    metadata.id,
                )?,
                metadata,
            })
        })
        .try_collect()
        .await?;

    Ok(Json(SearchResponse { images, pages }))
}

async fn update(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    req: Option<Json<<endpoints::image::UpdateMetadata as ApiEndpoint>::Req>>,
    id: Path<ImageId>,
) -> Result<HttpResponse, <endpoints::image::UpdateMetadata as ApiEndpoint>::Err> {
    let req = req.map_or_else(UpdateRequest::default, Json::into_inner);
    let id = id.into_inner();
    let mut txn = db.begin().await?;

    let exists = db::image::update(
        &mut txn,
        id,
        req.name.as_deref(),
        req.description.as_deref(),
        req.is_premium,
        req.publish_at.map(|it| it.map(DateTime::<Utc>::from)),
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

    txn.commit().await?;

    Ok(HttpResponse::NoContent().into())
}

fn check_conflict_delete(err: sqlx::Error) -> DeleteError {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            DeleteError::Conflict
        }
        _ => DeleteError::InternalServerError(err.into()),
    }
}

async fn delete(
    db: Data<PgPool>,
    algolia: Data<AlgoliaClient>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    req: Path<ImageId>,
    s3: Data<S3Client>,
) -> Result<HttpResponse, <endpoints::image::user::Delete as ApiEndpoint>::Err> {
    let image = req.into_inner();
    db::image::delete(&db, image)
        .await
        .map_err(check_conflict_delete)?;

    let delete_image = |kind| s3.delete_image(S3LibraryKind::Global, kind, image);
    let ((), (), (), ()) = futures::future::join4(
        delete_image(S3MediaVariant::Original),
        delete_image(S3MediaVariant::Resized),
        delete_image(S3MediaVariant::Thumbnail),
        algolia.delete_image(image),
    )
    .await;

    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}

pub fn configure(cfg: &mut ServiceConfig) {
    use endpoints::image;
    cfg.route(
        image::Create::PATH,
        image::Create::METHOD.route().to(create),
    )
    .service(
        web::resource(image::Upload::PATH)
            .app_data(PayloadConfig::default().limit(config::IMAGE_BODY_SIZE_LIMIT))
            .route(image::Upload::METHOD.route().to(upload)),
    )
    .route(image::Get::PATH, image::Get::METHOD.route().to(get_one))
    .route(image::Search::PATH, image::Search::METHOD.route().to(get))
    .route(
        image::UpdateMetadata::PATH,
        image::UpdateMetadata::METHOD.route().to(update),
    )
    .route(
        image::Delete::PATH,
        image::Delete::METHOD.route().to(delete),
    )
    .route(
        image::user::Create::PATH,
        image::user::Create::METHOD.route().to(self::user::create),
    )
    .route(
        image::user::Upload::PATH,
        image::user::Upload::METHOD.route().to(self::user::upload),
    )
    .route(
        image::user::Delete::PATH,
        image::user::Delete::METHOD.route().to(self::user::delete),
    )
    .route(
        image::user::Get::PATH,
        image::user::Get::METHOD.route().to(self::user::get),
    )
    .route(
        image::user::List::PATH,
        image::user::List::METHOD.route().to(self::user::list),
    );
}
