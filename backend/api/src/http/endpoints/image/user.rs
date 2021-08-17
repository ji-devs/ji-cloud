use futures::TryStreamExt;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Path, Query},
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
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{RequestOrigin, TokenUser},
    service::{s3, storage, GcpAccessKeyStore, ServiceData},
};

/// Create a image in the user's image library.
#[api_v2_operation]
pub(super) async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    query: Json<<endpoints::image::user::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<<endpoints::image::user::Create as ApiEndpoint>::Res>, error::Server> {
    let kind = query.kind;

    let user_id = claims.0.user_id;

    let id = db::image::user::create(db.as_ref(), &user_id, kind).await?;

    Ok(CreatedJson(CreateResponse { id }))
}

/// Upload an image to the user's image library.
#[api_v2_operation]
pub(super) async fn upload(
    db: Data<PgPool>,
    gcp_key_store: ServiceData<GcpAccessKeyStore>,
    gcs: ServiceData<storage::Client>,
    claims: TokenUser,
    Path(id): Path<ImageId>,
    origin: RequestOrigin,
    req: Json<<endpoints::image::user::Upload as ApiEndpoint>::Req>,
) -> Result<Json<<endpoints::image::user::Upload as ApiEndpoint>::Res>, error::Upload> {
    let mut txn = db.begin().await?;

    db::image::user::auth_user_image(&mut txn, &claims.0.user_id, &id).await?;

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
    claims: TokenUser,
    req: Path<ImageId>,
    s3: ServiceData<s3::Client>,
) -> Result<NoContent, error::Delete> {
    let id = req.into_inner();

    db::image::user::delete(&db, claims.0.user_id, id)
        .await
        .map_err(super::check_conflict_delete)?;

    let delete = |kind| s3.delete_media(MediaLibrary::User, FileKind::ImagePng(kind), id.0);
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
    claims: TokenUser,
    req: Path<ImageId>,
) -> Result<Json<<endpoints::image::user::Get as ApiEndpoint>::Res>, error::NotFound> {
    let image_id = req.into_inner();

    let metadata = db::image::user::get(&db, claims.0.user_id, image_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(UserImageResponse { metadata }))
}

/// List images from the user's image library.
#[api_v2_operation]
pub(super) async fn list(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Query<<endpoints::image::user::List as ApiEndpoint>::Req>,
) -> Result<Json<<endpoints::image::user::List as ApiEndpoint>::Res>, error::Server> {
    let images: Vec<_> = db::image::user::list(db.as_ref(), claims.0.user_id, req.kind)
        .err_into::<error::Server>()
        .and_then(|metadata: UserImage| async { Ok(UserImageResponse { metadata }) })
        .try_collect()
        .await?;

    Ok(Json(UserImageListResponse { images }))
}
