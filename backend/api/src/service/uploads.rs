use crate::{error, service};
use shared::{
    domain::{animation::AnimationKind, image::ImageKind},
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn process_image(db: &PgPool, s3: &crate::s3::Client, id: Uuid) -> anyhow::Result<bool> {
    log::info!("Processing image {}", id);

    let mut txn = db.begin().await?;

    let kind = sqlx::query!(
        r#"
select kind as "kind: ImageKind"
from image_metadata
inner join image_upload on image_metadata.id = image_upload.image_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of image_upload
for share of image_metadata
skip locked
        "#,
        id
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.kind);

    let kind = match kind {
        Some(row) => row,
        None => {
            log::info!("Unprocessed image upload not found in database!");

            txn.rollback().await?;
            return Ok(false);
        }
    };

    let file = s3
        .download_media_for_processing(
            MediaLibrary::Global,
            id,
            FileKind::ImagePng(PngImageFile::Original),
        )
        .await?;

    let file = match file {
        Some(it) => it,
        None => {
            sqlx::query!("update image_upload set processed_at = now(), processing_result = false where image_id = $1", id)
                .execute(&mut txn)
                .await?;

            log::warn!("Image wasn't uploaded properly before processing?");
            txn.commit().await?;
            return Ok(true);
        }
    };

    let processed = tokio::task::spawn_blocking(move || -> Result<_, error::Upload> {
        let original = image::load_from_memory(&file).map_err(|_| error::Upload::InvalidMedia)?;
        Ok(crate::image_ops::regenerate_images(&original, kind)?)
    })
    .await
    .unwrap();

    let (resized, thumbnail) = match processed {
        Ok(it) => it,
        Err(error::Upload::InvalidMedia) => {
            log::info!("invalid media");
            sqlx::query!("update image_upload set processed_at = now(), processing_result = false where image_id = $1", id)
                .execute(&mut txn)
                .await?;

            txn.commit().await?;
            return Ok(true);
        }
        Err(error::Upload::InternalServerError(e)) => return Err(e),
        Err(_) => unreachable!(),
    };

    s3.upload_png_images_copy_original(MediaLibrary::Global, id, resized, thumbnail)
        .await?;

    sqlx::query!("update image_upload set processed_at = now(), processing_result = true where image_id = $1", id)
        .execute(&mut txn)
        .await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn process_user_image(
    db: &PgPool,
    s3: &crate::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        r#"
select exists(select 1
from user_image_library
inner join user_image_upload on user_image_library.id = user_image_upload.image_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of user_image_upload
for share of user_image_library
skip locked
) as "exists!"
        "#,
        id
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if !exists {
        txn.rollback().await?;
        return Ok(false);
    }

    let file = s3
        .download_media_for_processing(
            MediaLibrary::User,
            id,
            FileKind::ImagePng(PngImageFile::Original),
        )
        .await?;

    let file = match file {
        Some(it) => it,
        None => {
            sqlx::query!("update user_image_upload set processed_at = now(), processing_result = false where image_id = $1", id)
                .execute(&mut txn)
                .await?;

            log::warn!("Image wasn't uploaded properly before processing?");
            txn.commit().await?;
            return Ok(true);
        }
    };

    let processed = tokio::task::spawn_blocking(move || -> Result<_, error::Upload> {
        let original = image::load_from_memory(&file).map_err(|_| error::Upload::InvalidMedia)?;
        Ok(crate::image_ops::regenerate_images(
            &original,
            ImageKind::Sticker,
        )?)
    })
    .await
    .unwrap();

    let (resized, thumbnail) = match processed {
        Ok(it) => it,
        Err(error::Upload::InvalidMedia) => {
            sqlx::query!("update user_image_upload set processed_at = now(), processing_result = false where image_id = $1", id)
                .execute(&mut txn)
                .await?;

            txn.commit().await?;
            return Ok(true);
        }
        Err(error::Upload::InternalServerError(e)) => return Err(e),
        Err(_) => unreachable!(),
    };

    s3.upload_png_images_copy_original(MediaLibrary::User, id, resized, thumbnail)
        .await?;

    sqlx::query!("update user_image_upload set processed_at = now(), processing_result = true where image_id = $1", id).execute(&mut txn).await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn process_animation(
    db: &PgPool,
    s3: &crate::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let row = sqlx::query!(
        r#"
select id,  kind as "kind: AnimationKind"
from animation_metadata
inner join global_animation_upload on animation_metadata.id = global_animation_upload.animation_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of global_animation_upload
for share of animation_metadata
skip locked
"#,
        id
    )
    .fetch_optional(&mut txn)
    .await?;

    let row = match row {
        Some(row) => row,
        None => {
            txn.rollback().await?;
            return Ok(false);
        }
    };

    if !matches!(row.kind, AnimationKind::Gif) {
        return Err(anyhow::anyhow!("Unimplemented Animation Kind: {:?}", row.kind).into());
    }

    let file = s3
        .download_media_for_processing(MediaLibrary::Global, id, FileKind::AnimationGif)
        .await?;

    let file = match file {
        Some(it) => it,
        None => {
            sqlx::query!("update global_animation_upload set processed_at = now(), processing_result = false where animation_id = $1", id)
                .execute(&mut txn)
                .await?;

            log::warn!("Animation wasn't uploaded properly before processing?");
            txn.commit().await?;
            return Ok(true);
        }
    };

    let res = tokio::task::spawn_blocking(move || -> Result<_, error::Upload> {
        let _ = image::load_from_memory_with_format(&file, image::ImageFormat::Gif)
            .or(Err(error::Upload::InvalidMedia))?;
        Ok(())
    })
    .await
    .unwrap();

    match res {
        Ok(()) => {}
        Err(error::Upload::InvalidMedia) => {
            sqlx::query!("update global_animation_upload set processed_at = now(), processing_result = false where animation_id = $1", id)
                .execute(&mut txn)
                .await?;

            txn.commit().await?;
            return Ok(true);
        }
        Err(error::Upload::InternalServerError(e)) => return Err(e),
        Err(_) => unreachable!(),
    };

    s3.copy_processed_file(MediaLibrary::Global, id, FileKind::AnimationGif)
        .await?;

    sqlx::query!("update global_animation_upload set processed_at = now(), processing_result = true where animation_id = $1", id).execute(&mut txn).await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn finalize_upload(
    access_token: &str,
    notifications: &service::notifications::Client,
    library: MediaLibrary,
    id: Uuid,
) -> anyhow::Result<()> {
    notifications
        .signal_status_ready(access_token, library, &id)
        .await?;
    Ok(())
}
