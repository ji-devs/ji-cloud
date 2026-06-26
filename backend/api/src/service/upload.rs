use shared::{
    domain::{
        animation::AnimationKind,
        image::{ImageFileKind, ImageSize},
    },
    media::{FileKind, MediaLibrary},
};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{error, service};

pub mod cleaner;

pub async fn process_image_bytes(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    id: Uuid,
    size: ImageSize,
    file: Vec<u8>,
) -> Result<(), error::Upload> {
    let kind = tokio::task::spawn_blocking({
        let file = file.clone();
        move || crate::image_ops::detect_image_kind(&file)
    })
    .await
    .unwrap()?;

    if matches!(
        kind.to_shared(),
        shared::media::MediaKind::Animation(AnimationKind::Gif)
    ) {
        if size != ImageSize::Sticker {
            return Err(error::Upload::InvalidMedia);
        }

        process_uploaded_gif(
            txn,
            s3,
            MediaLibrary::Global,
            id,
            file,
            "image_upload",
            "image_id",
        )
        .await?;
        return Ok(());
    }

    let processed = tokio::task::spawn_blocking(move || -> Result<_, error::Upload> {
        let original = image::load_from_memory(&file).map_err(|_| error::Upload::InvalidMedia)?;
        Ok(crate::image_ops::generate_images(&original, size)?)
    })
    .await
    .unwrap()?;

    let (original, resized, thumbnail) = processed;
    s3.upload_png_images(MediaLibrary::Global, id, original, resized, thumbnail)
        .await?;

    sqlx::query!(
        "update image_upload set uploaded_at = now(), processed_at = now(), processing_result = true, kind = $2 where image_id = $1",
        id,
        ImageFileKind::Png as i16,
    )
    .execute(&mut *txn)
    .await?;

    Ok(())
}

pub async fn process_user_image_bytes(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    id: Uuid,
    size: ImageSize,
    file: Vec<u8>,
) -> Result<(), error::Upload> {
    let kind = tokio::task::spawn_blocking({
        let file = file.clone();
        move || crate::image_ops::detect_image_kind(&file)
    })
    .await
    .unwrap()?;

    if matches!(
        kind.to_shared(),
        shared::media::MediaKind::Animation(AnimationKind::Gif)
    ) {
        process_uploaded_gif(
            txn,
            s3,
            MediaLibrary::User,
            id,
            file,
            "user_image_upload",
            "image_id",
        )
        .await?;
        return Ok(());
    }

    let processed = tokio::task::spawn_blocking(move || -> Result<_, error::Upload> {
        let original = image::load_from_memory(&file).map_err(|_| error::Upload::InvalidMedia)?;
        Ok(crate::image_ops::generate_images(&original, size)?)
    })
    .await
    .unwrap()?;

    let (original, resized, thumbnail) = processed;
    s3.upload_png_images(MediaLibrary::User, id, original, resized, thumbnail)
        .await?;

    sqlx::query!(
        "update user_image_upload set uploaded_at = now(), processed_at = now(), processing_result = true, kind = $2 where image_id = $1",
        id,
        ImageFileKind::Png as i16,
    )
    .execute(&mut *txn)
    .await?;

    Ok(())
}

async fn process_uploaded_gif(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    library: MediaLibrary,
    id: Uuid,
    file: Vec<u8>,
    table: &'static str,
    id_column: &'static str,
) -> Result<(), error::Upload> {
    validate_gif(file.clone()).await?;

    s3.upload_media(file, library, id, FileKind::AnimationGif)
        .await?;

    let query = format!(
        "update {table} set uploaded_at = now(), processed_at = now(), processing_result = true, kind = $2 where {id_column} = $1"
    );
    sqlx::query(&query)
        .bind(id)
        .bind(ImageFileKind::Gif as i16)
        .execute(&mut *txn)
        .await?;

    Ok(())
}

async fn validate_gif(file: Vec<u8>) -> Result<(), error::Upload> {
    tokio::task::spawn_blocking(move || -> Result<(), error::Upload> {
        image::load_from_memory_with_format(&file, image::ImageFormat::Gif)
            .map_err(|_| error::Upload::InvalidMedia)?;
        Ok(())
    })
    .await
    .unwrap()
}

pub async fn process_animation_bytes(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    id: Uuid,
    file: Vec<u8>,
) -> Result<(), error::Upload> {
    validate_gif(file.clone()).await?;

    s3.upload_media(file, MediaLibrary::Global, id, FileKind::AnimationGif)
        .await?;

    sqlx::query!(
        "update global_animation_upload set uploaded_at = now(), processed_at = now(), processing_result = true where animation_id = $1",
        id
    )
    .execute(&mut *txn)
    .await?;

    Ok(())
}

pub async fn process_user_audio_bytes(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    id: Uuid,
    file: Vec<u8>,
) -> Result<(), error::Upload> {
    s3.upload_media(file, MediaLibrary::User, id, FileKind::AudioMp3)
        .await?;

    sqlx::query!(
        "update user_audio_upload set uploaded_at = now(), processed_at = now(), processing_result = true where audio_id = $1",
        id
    )
    .execute(&mut *txn)
    .await?;

    Ok(())
}

pub async fn process_user_pdf_bytes(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    id: Uuid,
    file: Vec<u8>,
) -> Result<(), error::Upload> {
    s3.upload_media(file, MediaLibrary::User, id, FileKind::DocumentPdf)
        .await?;

    sqlx::query!(
        "update user_pdf_upload set uploaded_at = now(), processed_at = now(), processing_result = true where pdf_id = $1",
        id
    )
    .execute(&mut *txn)
    .await?;

    Ok(())
}
