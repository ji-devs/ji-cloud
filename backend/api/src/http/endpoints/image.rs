use actix_web::{
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::{
        CreateResponse, ImageBrowseResponse, ImageId, ImageMetadata, ImageResponse,
        ImageSearchResponse, ImageUpdateRequest, ImageUploadResponse,
    },
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::{postgres::PgDatabaseError, PgPool};

use crate::{
    db::{self, meta::handle_metadata_err, nul_if_empty},
    error::{self, ServiceKind},
    extractor::{RequestOrigin, ScopeManageImage, TokenUser, TokenUserWithScope},
    service::{self, s3, ServiceData},
};

pub mod recent;
pub mod tag;
pub mod user;

const MAX_PAGE_LIMIT: u32 = 100;
const DEFAULT_PAGE_LIMIT: u32 = 20;

/// Create an image in the global image library.
async fn create(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageImage>,
    req: Json<<endpoints::image::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::CreateWithMetadata> {
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
        nul_if_empty(&req.tags),
    )
    .await
    .map_err(handle_metadata_err)?;

    txn.commit().await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Upload an image to the global image library.
async fn upload(
    db: Data<PgPool>,
    gcp_key_store: ServiceData<service::GcpAccessKeyStore>,
    gcs: ServiceData<service::storage::Client>,
    _claims: TokenUserWithScope<ScopeManageImage>,
    path: Path<ImageId>,
    origin: RequestOrigin,
    req: Json<<endpoints::image::Upload as ApiEndpoint>::Req>,
) -> Result<Json<<endpoints::image::Upload as ApiEndpoint>::Res>, error::Upload> {
    let mut txn = db.begin().await?;

    let id = path.into_inner();

    let exists = sqlx::query!(
        r#"select exists(select 1 from image_upload where image_id = $1 for no key update) as "exists!""#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?.exists;

    if !exists {
        return Err(error::Upload::ResourceNotFound);
    }

    let upload_content_length = req.into_inner().file_size;

    if let Some(file_limit) = gcs.file_size_limit(&FileKind::ImagePng(PngImageFile::Original)) {
        if file_limit < upload_content_length {
            return Err(error::Upload::FileTooLarge);
        }
    }

    let access_token = gcp_key_store.fetch_token().await?.to_owned();

    let resp = gcs
        .get_url_for_resumable_upload_for_processing(
            &access_token,
            upload_content_length,
            MediaLibrary::Global,
            id.0,
            FileKind::ImagePng(PngImageFile::Original),
            origin,
        )
        .await?;

    sqlx::query!(
        "update image_upload set uploaded_at = now(), processing_result = null where image_id = $1",
        id.0
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(Json(ImageUploadResponse { session_uri: resp }))
}

/// Get an image from the global image library.
async fn get_one(
    db: Data<PgPool>,
    _claims: TokenUser,
    req: Path<ImageId>,
) -> Result<Json<<endpoints::image::Get as ApiEndpoint>::Res>, error::NotFound> {
    let metadata = db::image::get_one(&db, req.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(ImageResponse { metadata }))
}

/// Search for images in the global image library.
async fn search(
    db: Data<PgPool>,
    algolia: ServiceData<crate::algolia::Client>,
    _claims: TokenUser,
    query: Option<Query<<endpoints::image::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<endpoints::image::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let (ids, pages, total_hits) = algolia
        .search_image(
            &query.q,
            query.kind,
            query.page,
            query.is_premium,
            query.is_published,
            &query.styles,
            &query.age_ranges,
            &query.affiliations,
            &query.categories,
            &query.tags,
            &query.tags_priority,
            page_limit,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let images: Vec<_> = db::image::get(db.as_ref(), &ids)
        .err_into::<error::Service>()
        .and_then(|metadata: ImageMetadata| async { Ok(ImageResponse { metadata }) })
        .try_collect()
        .await?;

    Ok(Json(ImageSearchResponse {
        images,
        pages,
        total_image_count: total_hits,
    }))
}

async fn browse(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageImage>,
    query: Option<Query<<endpoints::image::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<endpoints::image::Browse as ApiEndpoint>::Res>, error::Server> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Server(e))?;

    let images: Vec<_> = db::image::list(
        db.as_ref(),
        query.is_published,
        query.kind,
        query.page.unwrap_or(0) as i32,
        page_limit,
    )
    .err_into::<error::Server>()
    .and_then(|metadata: ImageMetadata| async { Ok(ImageResponse { metadata }) })
    .try_collect()
    .await?;

    log::warn!("images: {:?}", images.len());

    let total_count =
        db::image::filtered_count(db.as_ref(), query.is_published, query.kind).await?;

    let pages = (total_count / (page_limit as u64)
        + (total_count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(ImageBrowseResponse {
        images,
        pages,
        total_image_count: total_count,
    }))
}

/// Update an image in the global image library.
async fn update(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageImage>,
    req: Option<Json<<endpoints::image::UpdateMetadata as ApiEndpoint>::Req>>,
    id: Path<ImageId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
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
        return Err(error::UpdateWithMetadata::ResourceNotFound);
    }

    db::image::update_metadata(
        &mut txn,
        id,
        req.affiliations.as_deref(),
        req.age_ranges.as_deref(),
        req.styles.as_deref(),
        req.categories.as_deref(),
        req.tags.as_deref(),
    )
    .await
    .map_err(handle_metadata_err)?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

fn check_conflict_delete(err: sqlx::Error) -> error::Delete {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            error::Delete::Conflict
        }
        _ => error::Delete::InternalServerError(err.into()),
    }
}

/// Delete an image from the global image library.
async fn delete(
    db: Data<PgPool>,
    algolia: ServiceData<crate::algolia::Client>,
    _claims: TokenUserWithScope<ScopeManageImage>,
    req: Path<ImageId>,
    s3: ServiceData<s3::Client>,
) -> Result<HttpResponse, error::Delete> {
    let image = req.into_inner();
    db::image::delete(&db, image)
        .await
        .map_err(check_conflict_delete)?;

    // todo: 501 when algolia is disabled.

    let delete = |kind| s3.delete_media(MediaLibrary::Global, FileKind::ImagePng(kind), image.0);
    let ((), (), (), ()) = futures::future::join4(
        delete(PngImageFile::Original),
        delete(PngImageFile::Resized),
        delete(PngImageFile::Thumbnail),
        algolia.delete_image(image),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

async fn page_limit(page_limit: Option<u32>) -> anyhow::Result<u32> {
    if let Some(limit) = page_limit {
        match limit > 0 && limit <= MAX_PAGE_LIMIT {
            true => {
                log::warn!("page limit: {:?}", limit);
                Ok(limit)
            }
            false => Err(anyhow::anyhow!("Page limit should be within 1-100")),
        }
    } else {
        Ok(DEFAULT_PAGE_LIMIT)
    }
}

pub fn configure(cfg: &mut ServiceConfig) {
    use endpoints::image;
    cfg.route(
        image::Create::PATH,
        image::Create::METHOD.route().to(create),
    )
    .route(
        image::Search::PATH,
        image::Search::METHOD.route().to(search),
    )
    .route(
        image::Browse::PATH,
        image::Browse::METHOD.route().to(browse),
    )
    .route(image::Get::PATH, image::Get::METHOD.route().to(get_one))
    .route(
        image::UpdateMetadata::PATH,
        image::UpdateMetadata::METHOD.route().to(update),
    )
    .route(
        image::Delete::PATH,
        image::Delete::METHOD.route().to(delete),
    )
    .route(
        image::Upload::PATH,
        image::Upload::METHOD.route().to(upload),
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
        image::tag::Create::PATH,
        image::tag::Create::METHOD.route().to(self::tag::create),
    )
    .route(
        image::tag::Update::PATH,
        image::tag::Update::METHOD.route().to(self::tag::update),
    )
    .route(
        image::tag::Delete::PATH,
        image::tag::Delete::METHOD.route().to(self::tag::delete),
    )
    .route(
        image::tag::List::PATH,
        image::tag::List::METHOD.route().to(self::tag::list),
    )
    .route(
        image::recent::List::PATH,
        image::recent::List::METHOD.route().to(self::recent::list),
    )
    .route(
        image::recent::Put::PATH,
        image::recent::Put::METHOD.route().to(self::recent::put),
    )
    .route(
        image::recent::Delete::PATH,
        image::recent::Delete::METHOD
            .route()
            .to(self::recent::delete),
    );
}
