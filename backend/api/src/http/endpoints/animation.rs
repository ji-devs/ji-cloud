use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use shared::{
    api::{endpoints::animation, ApiEndpoint, PathParts},
    domain::{
        animation::{AnimationId, AnimationKind, AnimationResponse, AnimationUploadResponse},
        CreateResponse,
    },
    media::{FileKind, MediaLibrary},
};
use sqlx::{postgres::PgDatabaseError, PgPool};

use crate::extractor::{RequestOrigin, ScopeManageAnimation, TokenUser, TokenUserWithScope};
use crate::service::{s3, storage, GcpAccessKeyStore, ServiceData};
use crate::{db, error};

fn check_conflict_delete(err: sqlx::Error) -> error::Delete {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            error::Delete::Conflict
        }
        _ => error::Delete::InternalServerError(err.into()),
    }
}

/// Delete an animation from the global animation library.
async fn delete(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageAnimation>,
    req: Path<AnimationId>,
    s3: ServiceData<s3::Client>,
) -> Result<HttpResponse, error::Delete> {
    let animation = req.into_inner();
    let kind = db::animation::delete(&db, animation)
        .await
        .map_err(check_conflict_delete)?;

    if let Some(kind) = kind {
        let file = match kind {
            AnimationKind::Gif => FileKind::AnimationGif,
            // todo:
            _ => return Err(anyhow::anyhow!("Unsupported animation kind").into()),
        };

        s3.delete_media(MediaLibrary::Global, file, animation.0)
            .await;
    }

    Ok(HttpResponse::NoContent().finish())
}

/// Create an animation in the global animation library.
async fn create(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageAnimation>,
    req: Json<<animation::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::CreateWithMetadata> {
    let req = req.into_inner();

    let mut txn = db.begin().await?;
    let id = db::animation::create(
        &mut txn,
        &req.name,
        &req.description,
        req.is_premium,
        req.is_looping,
        req.publish_at.map(DateTime::<Utc>::from),
        req.kind,
    )
    .await?;

    // todo: have these exist
    // db::animation::update_metadata(
    //     &mut txn,
    //     id,
    //     nul_if_empty(&req.affiliations),
    //     nul_if_empty(&req.age_ranges),
    //     nul_if_empty(&req.styles),
    //     nul_if_empty(&req.categories),
    // )
    // .await
    // .map_err(handle_metadata_err)?;

    txn.commit().await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Upload an animation to the global animation library.
async fn upload(
    db: Data<PgPool>,
    gcp_key_store: ServiceData<GcpAccessKeyStore>,
    gcs: ServiceData<storage::Client>,
    _claims: TokenUserWithScope<ScopeManageAnimation>,
    path: Path<AnimationId>,
    origin: RequestOrigin,
    req: Json<<animation::Upload as ApiEndpoint>::Req>,
) -> Result<Json<<animation::Upload as ApiEndpoint>::Res>, error::Upload> {
    let mut txn = db.begin().await?;

    let id = path.into_inner();

    let exists = sqlx::query!(
        r#"select exists(select 1 from global_animation_upload where animation_id = $1 for no key update) as "exists!""#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?.exists;

    if !exists {
        return Err(error::Upload::ResourceNotFound);
    }

    let upload_content_length = req.into_inner().file_size;

    if let Some(file_limit) = gcs.file_size_limit(&FileKind::AnimationGif) {
        if file_limit < upload_content_length {
            return Err(error::Upload::FileTooLarge);
        }
    }

    let access_token = gcp_key_store.fetch_token().await?;

    let resp = gcs
        .get_url_for_resumable_upload_for_processing(
            &access_token,
            upload_content_length,
            MediaLibrary::Global,
            id.0,
            FileKind::AnimationGif,
            origin,
        )
        .await?;

    sqlx::query!(
        "update global_animation_upload set uploaded_at = now(), processing_result = null where animation_id = $1",
        id.0
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(Json(AnimationUploadResponse { session_uri: resp }))
}

/// Get an animation from the global animation library.
async fn get_one(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: Path<AnimationId>,
) -> Result<Json<<animation::Get as ApiEndpoint>::Res>, error::NotFound> {
    let metadata = db::animation::get_one(&db, path.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(AnimationResponse { metadata }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <animation::Create as ApiEndpoint>::Path::PATH,
        animation::Create::METHOD.route().to(create),
    )
    .route(
        <animation::Upload as ApiEndpoint>::Path::PATH,
        animation::Upload::METHOD.route().to(upload),
    )
    .route(
        <animation::Get as ApiEndpoint>::Path::PATH,
        animation::Get::METHOD.route().to(get_one),
    )
    // .route(
    //     <animation::UpdateMetadata as ApiEndpoint>::Path::PATH,
    //     animation::UpdateMetadata::METHOD.route().to(update),
    // )
    .route(
        <animation::Delete as ApiEndpoint>::Path::PATH,
        animation::Delete::METHOD.route().to(delete),
    );
}
