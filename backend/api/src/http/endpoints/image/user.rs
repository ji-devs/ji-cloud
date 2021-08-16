use crate::extractor::RequestOrigin;
use crate::service::storage;
use crate::{
    db, error,
    extractor::TokenUser,
    s3,
    service::{GcpAccessKeyStore, ServiceData},
};

use futures::TryStreamExt;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Path},
    CreatedJson, NoContent,
};
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        image::{
            user::{UserImage, UserImageListResponse, UserImageResponse, UserImageUploadResponse},
            ImageId,
        },
        CreateResponse,
    },
    media::MediaLibrary,
    media::{FileKind, PngImageFile},
};
use sqlx::PgPool;

/// Create a image in the user's image library.
#[api_v2_operation]
pub(super) async fn create(
    db: Data<PgPool>,
    _claims: TokenUser,
) -> Result<CreatedJson<<endpoints::image::user::Create as ApiEndpoint>::Res>, error::Server> {
    let id = db::image::user::create(db.as_ref()).await?;
    Ok(CreatedJson(CreateResponse { id }))
}

/// Upload an image to the user's image library.
#[api_v2_operation]
pub(super) async fn upload(
    db: Data<PgPool>,
    gcp_key_store: ServiceData<GcpAccessKeyStore>,
    gcs: ServiceData<storage::Client>,
    _claims: TokenUser,
    Path(id): Path<ImageId>,
    origin: RequestOrigin,
    req: Json<<endpoints::image::user::Upload as ApiEndpoint>::Req>,
) -> Result<Json<<endpoints::image::user::Upload as ApiEndpoint>::Res>, error::Upload> {
    let mut txn = db.begin().await?;

    sqlx::query!(
            r#"select exists(select 1 from user_image_upload where image_id = $1 for no key update) as "exists!""#,
                id.0
        )
        .fetch_optional(&mut txn)
        .await?
        .ok_or(error::Upload::ResourceNotFound)?;

    let upload_content_length = req.into_inner().file_size;

    if let Some(file_limit) = gcs.file_size_limit(&FileKind::ImagePng(PngImageFile::Original)) {
        if file_limit < upload_content_length {
            return Err(error::Upload::FileTooLarge);
        }
    }

    let access_token = gcp_key_store.fetch_token().await?;

    let resp = gcs
        .get_url_for_resumable_upload_for_processing(
            &access_token,
            upload_content_length,
            MediaLibrary::User,
            id.0,
            FileKind::ImagePng(PngImageFile::Original),
            origin,
        )
        .await?;

    sqlx::query!(
            "update user_image_upload set uploaded_at = now(), processing_result = null where image_id = $1",
            id.0
        )
        .execute(&mut txn)
        .await?;

    txn.commit().await?;

    Ok(Json(UserImageUploadResponse { session_uri: resp }))
}

/// Delete an image from the user's image library.
#[api_v2_operation]
pub(super) async fn delete(
    db: Data<PgPool>,
    _claims: TokenUser,
    req: Path<ImageId>,
    s3: ServiceData<s3::Client>,
) -> Result<NoContent, error::Delete> {
    let image = req.into_inner();
    db::image::user::delete(&db, image)
        .await
        .map_err(super::check_conflict_delete)?;

    let delete = |kind| s3.delete_media(MediaLibrary::User, FileKind::ImagePng(kind), image.0);
    let ((), (), ()) = futures::future::join3(
        delete(PngImageFile::Original),
        delete(PngImageFile::Resized),
        delete(PngImageFile::Thumbnail),
    )
    .await;

    Ok(NoContent)
}

/// Get an image from the user's image library.
#[api_v2_operation]
pub(super) async fn get(
    db: Data<PgPool>,
    _claims: TokenUser,
    req: Path<ImageId>,
) -> Result<Json<<endpoints::image::user::Get as ApiEndpoint>::Res>, error::NotFound> {
    let metadata = db::image::user::get(&db, req.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(UserImageResponse { metadata }))
}

/// List images from the user's image library.
#[api_v2_operation]
pub(super) async fn list(
    db: Data<PgPool>,
    _claims: TokenUser,
) -> Result<Json<<endpoints::image::user::List as ApiEndpoint>::Res>, error::Server> {
    let images: Vec<_> = db::image::user::list(db.as_ref())
        .err_into::<error::Server>()
        .and_then(|metadata: UserImage| async { Ok(UserImageResponse { metadata }) })
        .try_collect()
        .await?;

    Ok(Json(UserImageListResponse { images }))
}
