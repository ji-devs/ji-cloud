use actix_web::{
    web::{Data, Json, Path, Payload, Query},
    HttpResponse,
};
use futures::TryStreamExt;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        image::{
            user::{UserImage, UserImageListResponse, UserImageResponse},
            ImageId, ImageSize,
        },
        CreateResponse,
    },
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::TokenUser,
    service::{s3, upload as upload_service, ServiceData},
};

/// Create a image in the user's image library.
pub(super) async fn create(
    db: Data<PgPool>,
    s3: ServiceData<s3::Client>,
    claims: TokenUser,
    query: Query<<endpoints::image::user::Create as ApiEndpoint>::Req>,
    payload: Payload,
) -> Result<HttpResponse, error::Upload> {
    let file =
        super::super::read_limited_payload(payload, FileKind::ImagePng(PngImageFile::Original))
            .await?;
    let size = query.size;

    let user_id = claims.user_id();

    let id = db::image::user::create(db.as_ref(), &user_id, size).await?;

    let mut txn = db.begin().await?;
    upload_service::process_user_image_bytes(&mut txn, &s3, id.0, size, file).await?;
    txn.commit().await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Upload an image to the user's image library.
pub(super) async fn upload(
    db: Data<PgPool>,
    s3: ServiceData<s3::Client>,
    claims: TokenUser,
    path: Path<ImageId>,
    payload: Payload,
) -> Result<HttpResponse, error::Upload> {
    let id = path.into_inner();
    let user_id = claims.user_id();
    let mut txn = db.begin().await?;

    db::image::user::auth_user_image(&mut txn, &user_id, &id).await?;

    let size = sqlx::query!(
        r#"select size as "size: ImageSize" from user_image_library where id = $1"#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?
    .size;

    let file =
        super::super::read_limited_payload(payload, FileKind::ImagePng(PngImageFile::Original))
            .await?;

    upload_service::process_user_image_bytes(&mut txn, &s3, id.0, size, file).await?;

    txn.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete an image from the user's image library.
pub(super) async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<ImageId>,
    s3: ServiceData<s3::Client>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();
    let user_id = claims.user_id();

    db::image::user::delete(&db, user_id, id)
        .await
        .map_err(super::check_conflict_delete)?;

    let delete = |kind| s3.delete_media(MediaLibrary::User, FileKind::ImagePng(kind), id.0);
    let ((), (), (), ()) = futures::future::join4(
        delete(PngImageFile::Original),
        delete(PngImageFile::Resized),
        delete(PngImageFile::Thumbnail),
        s3.delete_media(MediaLibrary::User, FileKind::AnimationGif, id.0),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

/// Get an image from the user's image library.
pub(super) async fn get(
    db: Data<PgPool>,
    claims: TokenUser,
    path: Path<ImageId>,
) -> Result<Json<<endpoints::image::user::Get as ApiEndpoint>::Res>, error::NotFound> {
    let image_id = path.into_inner();
    let user_id = claims.user_id();

    let metadata = db::image::user::get(&db, user_id, image_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(UserImageResponse { metadata }))
}

/// List images from the user's image library.
pub(super) async fn list(
    db: Data<PgPool>,
    claims: TokenUser,
    query: Query<<endpoints::image::user::List as ApiEndpoint>::Req>,
) -> Result<Json<<endpoints::image::user::List as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.user_id();

    let images: Vec<_> = db::image::user::list(db.as_ref(), user_id, query.kind)
        .err_into::<error::Server>()
        .and_then(|metadata: UserImage| async { Ok(UserImageResponse { metadata }) })
        .try_collect()
        .await?;

    Ok(Json(UserImageListResponse { images }))
}
