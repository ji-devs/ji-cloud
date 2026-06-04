use shared::{
    domain::image::ImageSize,
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
        "update image_upload set uploaded_at = now(), processed_at = now(), processing_result = true where image_id = $1",
        id
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
        "update user_image_upload set uploaded_at = now(), processed_at = now(), processing_result = true where image_id = $1",
        id
    )
    .execute(&mut *txn)
    .await?;

    Ok(())
}

pub async fn process_animation_bytes(
    txn: &mut Transaction<'_, Postgres>,
    s3: &service::s3::Client,
    id: Uuid,
    file: Vec<u8>,
) -> Result<(), error::Upload> {
    let validate_file = file.clone();
    tokio::task::spawn_blocking(move || -> Result<(), error::Upload> {
        image::load_from_memory_with_format(&validate_file, image::ImageFormat::Gif)
            .map_err(|_| error::Upload::InvalidMedia)?;
        Ok(())
    })
    .await
    .unwrap()?;

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
