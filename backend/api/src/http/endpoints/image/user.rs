use crate::{
    db, error, extractor::TokenUser, image_ops::generate_images, s3, service::ServiceData,
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
    s3: ServiceData<s3::Client>,
    _claims: TokenUser,
    Path(id): Path<ImageId>,
    bytes: Bytes,
) -> Result<NoContent, error::Upload> {
    let mut txn = db.begin().await?;

    sqlx::query!(
        r#"select 1 as discard from user_image_library where id = $1 for update"#,
        id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::Upload::ResourceNotFound)?;

    let kind = ImageKind::Sticker;

    let (original, resized, thumbnail) =
        actix_web::web::block(move || -> Result<_, error::Upload> {
            let original =
                image::load_from_memory(&bytes).map_err(|_| error::Upload::InvalidMedia)?;
            Ok(generate_images(&original, kind)?)
        })
        .await
        .map_err(error::Upload::blocking_error)?;

    s3.upload_png_images(MediaLibrary::User, id.0, original, resized, thumbnail)
        .await?;

    sqlx::query!(
        "update user_image_library set uploaded_at = now() where id = $1",
        id.0
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(NoContent)
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
