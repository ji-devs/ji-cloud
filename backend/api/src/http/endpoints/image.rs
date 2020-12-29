use crate::{
    algolia::AlgoliaClient,
    db::{self, meta::MetaWrapperError, nul_if_empty},
    error::{
        CreateWithMetadataError, DeleteError, NotFoundError, ServerError, UpdateWithMetadataError,
        UploadError,
    },
    extractor::{AuthUserWithScope, ScopeManageImage, WrapAuthClaimsNoDb},
    image_ops::generate_images,
    s3::S3Client,
};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use paperclip::actix::{
    api_v2_operation,
    web::{self, Bytes, Data, Json, Path, PayloadConfig, Query, ServiceConfig},
    CreatedJson, NoContent,
};
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        image::{
            CreateResponse, Image, ImageId, ImageResponse, ImageSearchResponse, ImageUpdateRequest,
        },
        meta::MetaKind,
    },
    media::{ImageVariant, MediaLibraryKind},
};
use sqlx::{postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

pub mod user {
    use crate::{
        db,
        error::{DeleteError, NotFoundError, ServerError, UploadError},
        extractor::WrapAuthClaimsNoDb,
        image_ops::generate_images,
        s3::S3Client,
    };
    use paperclip::actix::{
        api_v2_operation,
        web::{Bytes, Data, Json, Path},
        CreatedJson, NoContent,
    };

    use futures::TryStreamExt;
    use shared::{
        api::{endpoints, ApiEndpoint},
        domain::{
            image::{
                user::{UserImage, UserImageListResponse, UserImageResponse},
                ImageId, ImageKind,
            },
            CreateResponse,
        },
        media::ImageVariant,
        media::MediaLibraryKind,
    };
    use sqlx::PgPool;

    /// Create a image in the user's image library.
    #[api_v2_operation]
    pub(super) async fn create(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
    ) -> Result<CreatedJson<<endpoints::image::user::Create as ApiEndpoint>::Res>, ServerError>
    {
        let id = db::image::user::create(db.as_ref()).await?;
        Ok(CreatedJson(CreateResponse { id }))
    }

    /// Upload an image to the user's image library.
    #[api_v2_operation]
    pub(super) async fn upload(
        db: Data<PgPool>,
        s3: Data<S3Client>,
        _claims: WrapAuthClaimsNoDb,
        Path(id): Path<ImageId>,
        bytes: Bytes,
    ) -> Result<NoContent, UploadError> {
        if !db::image::user::exists(db.as_ref(), id).await? {
            return Err(UploadError::ResourceNotFound);
        }

        let kind = ImageKind::Sticker;

        let res: Result<_, UploadError> = tokio::task::spawn_blocking(move || {
            let original =
                image::load_from_memory(&bytes).map_err(|_| UploadError::InvalidMedia)?;
            Ok(generate_images(original, kind)?)
        })
        .await?;

        let (original, resized, thumbnail) = res?;
        s3.upload_images(MediaLibraryKind::User, id, original, resized, thumbnail)
            .await?;

        Ok(NoContent)
    }

    /// Delete an image from the user's image library.
    #[api_v2_operation]
    pub(super) async fn delete(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
        req: Path<ImageId>,
        s3: Data<S3Client>,
    ) -> Result<NoContent, DeleteError> {
        let image = req.into_inner();
        db::image::user::delete(&db, image)
            .await
            .map_err(super::check_conflict_delete)?;

        let delete_image = |kind| s3.delete_image(MediaLibraryKind::Global, kind, image);
        let ((), (), ()) = futures::future::join3(
            delete_image(ImageVariant::Original),
            delete_image(ImageVariant::Resized),
            delete_image(ImageVariant::Thumbnail),
        )
        .await;

        Ok(NoContent)
    }

    /// Get an image from the user's image library.
    #[api_v2_operation]
    pub(super) async fn get(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
        req: Path<ImageId>,
    ) -> Result<Json<<endpoints::image::user::Get as ApiEndpoint>::Res>, NotFoundError> {
        let metadata = db::image::user::get(&db, req.into_inner())
            .await?
            .ok_or(NotFoundError::ResourceNotFound)?;

        Ok(Json(UserImageResponse { metadata }))
    }

    /// List images from the user's image library.
    #[api_v2_operation]
    pub(super) async fn list(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
    ) -> Result<Json<<endpoints::image::user::List as ApiEndpoint>::Res>, ServerError> {
        let images: Vec<_> = db::image::user::list(db.as_ref())
            .err_into::<ServerError>()
            .and_then(|metadata: UserImage| async { Ok(UserImageResponse { metadata }) })
            .try_collect()
            .await?;

        Ok(Json(UserImageListResponse { images }))
    }
}

mod web_library {
    use core::settings::RuntimeSettings;

    use paperclip::actix::{
        api_v2_operation,
        web::{Data, Json, Query},
    };

    use shared::{
        api::{endpoints, ApiEndpoint},
        domain::image::web::WebImageSearchResponse,
    };

    use crate::{error::ServerError, extractor::WrapAuthClaimsNoDb};

    /// Search for images in the web image library.
    #[api_v2_operation]
    pub async fn search(
        runtime_settings: Data<RuntimeSettings>,
        _claims: WrapAuthClaimsNoDb,
        query: Query<<endpoints::image::web::Search as ApiEndpoint>::Req>,
    ) -> Result<Json<<endpoints::image::web::Search as ApiEndpoint>::Res>, ServerError> {
        let query = query.into_inner();

        // todo: handle empty queries (they're invalid in bing)

        let res = match &runtime_settings.bing_search_key {
            Some(key) => crate::image_search::get_images(&query.q, key).await?,
            None => WebImageSearchResponse { images: Vec::new() },
        };

        Ok(Json(res))
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

/// Create an image in the global image library.
#[api_v2_operation]
async fn create(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    req: Json<<endpoints::image::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<endpoints::image::Create as ApiEndpoint>::Res>, CreateWithMetadataError> {
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

    Ok(CreatedJson(CreateResponse { id }))
}

/// Upload an image to the global image library.
#[api_v2_operation]
async fn upload(
    db: Data<PgPool>,
    s3: Data<S3Client>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    Path(id): Path<ImageId>,
    bytes: Bytes,
) -> Result<NoContent, UploadError> {
    let kind = db::image::get_image_kind(db.as_ref(), id)
        .await?
        .ok_or(UploadError::ResourceNotFound)?;

    let res: Result<_, UploadError> = tokio::task::spawn_blocking(move || {
        let original = image::load_from_memory(&bytes).map_err(|_| UploadError::InvalidMedia)?;
        Ok(generate_images(original, kind)?)
    })
    .await?;

    let (original, resized, thumbnail) = res?;
    s3.upload_images(MediaLibraryKind::Global, id, original, resized, thumbnail)
        .await?;

    Ok(NoContent)
}

/// Get an image from the global image library.
#[api_v2_operation]
async fn get_one(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Path<ImageId>,
) -> Result<Json<<endpoints::image::Get as ApiEndpoint>::Res>, NotFoundError> {
    let metadata = db::image::get_one(&db, req.into_inner())
        .await?
        .ok_or(NotFoundError::ResourceNotFound)?;

    Ok(Json(ImageResponse { metadata }))
}

/// Search for images in the global image library.
#[api_v2_operation]
async fn search(
    db: Data<PgPool>,
    algolia: Data<AlgoliaClient>,
    _claims: WrapAuthClaimsNoDb,
    query: Option<Query<<endpoints::image::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<endpoints::image::Search as ApiEndpoint>::Res>, ServerError> {
    let query = dbg!(query.map_or_else(Default::default, Query::into_inner));

    let (ids, pages, total_hits) = algolia
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
        .err_into::<ServerError>()
        .and_then(|metadata: Image| async { Ok(ImageResponse { metadata }) })
        .try_collect()
        .await?;

    Ok(Json(ImageSearchResponse {
        images,
        pages,
        total_image_count: total_hits,
    }))
}

/// Update an image in the global image library.
#[api_v2_operation]
async fn update(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    req: Option<Json<<endpoints::image::UpdateMetadata as ApiEndpoint>::Req>>,
    id: Path<ImageId>,
) -> Result<NoContent, UpdateWithMetadataError> {
    let req = req.map_or_else(ImageUpdateRequest::default, Json::into_inner);
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
        return Err(UpdateWithMetadataError::ResourceNotFound);
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

    Ok(NoContent)
}

fn check_conflict_delete(err: sqlx::Error) -> DeleteError {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            DeleteError::Conflict
        }
        _ => DeleteError::InternalServerError(err.into()),
    }
}

/// Delete an image from the global image library.
#[api_v2_operation]
async fn delete(
    db: Data<PgPool>,
    algolia: Data<AlgoliaClient>,
    _claims: AuthUserWithScope<ScopeManageImage>,
    req: Path<ImageId>,
    s3: Data<S3Client>,
) -> Result<NoContent, DeleteError> {
    let image = req.into_inner();
    db::image::delete(&db, image)
        .await
        .map_err(check_conflict_delete)?;

    let delete_image = |kind| s3.delete_image(MediaLibraryKind::Global, kind, image);
    let ((), (), (), ()) = futures::future::join4(
        delete_image(ImageVariant::Original),
        delete_image(ImageVariant::Resized),
        delete_image(ImageVariant::Thumbnail),
        algolia.delete_image(image),
    )
    .await;

    Ok(NoContent)
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
    .route(
        image::Search::PATH,
        image::Search::METHOD.route().to(search),
    )
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
    )
    .route(
        image::web::Search::PATH,
        image::web::Search::METHOD
            .route()
            .to(self::web_library::search),
    );
}
