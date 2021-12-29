use shared::{
    domain::{animation::AnimationKind, image::ImageKind},
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error, image_ops::MediaKind, service};

pub mod cleaner;

pub async fn process_image(
    db: &PgPool,
    s3: &service::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
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
    s3: &service::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let kind = sqlx::query!(
        //language=SQL
        r#"
select kind as "kind: ImageKind"
from user_image_library
inner join user_image_upload on user_image_library.id = user_image_upload.image_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of user_image_upload
for share of user_image_library
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
            log::info!("Unprocessed user image upload not found in database!");

            txn.rollback().await?;
            return Ok(false);
        }
    };

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
        Ok(crate::image_ops::regenerate_images(&original, kind)?)
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

pub async fn process_web_media(
    db: &PgPool,
    s3: &service::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let kind = sqlx::query!(
        //language=SQL
        r#"
select kind as "kind: MediaKind"
from web_media_library
inner join web_media_upload on web_media_library.id = web_media_upload.media_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of web_media_upload
for share of web_media_library
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
            log::info!("Unprocessed web media upload not found in database!");

            txn.rollback().await?;
            return Ok(false);
        }
    };

    // download raw media with ID
    // Upload processed file to firestone by kind
    match kind {
        MediaKind::GifAnimation => {
            let file = s3
                .download_media_for_processing(MediaLibrary::Web, id, FileKind::AnimationGif)
                .await?;

            if let Some(data) = file {
                s3.upload_media(data, MediaLibrary::Web, id, FileKind::AnimationGif)
                    .await?;
            }
        }

        MediaKind::PngStickerImage => {
            let file = s3
                .download_media_for_processing(
                    MediaLibrary::Web,
                    id,
                    FileKind::ImagePng(PngImageFile::Original),
                )
                .await?;

            if let Some(data) = file {
                let (_original, resized, thumbnail) = actix_web::web::block(move || {
                    let original = image::load_from_memory(&data)?;
                    crate::image_ops::generate_images(&original, ImageKind::Sticker)
                })
                .await??;

                s3.upload_png_images_copy_original(MediaLibrary::Web, id, resized, thumbnail)
                    .await?;
            }
        }

        kind => return Err(anyhow::anyhow!("unsupported media kind {:?}", kind).into()),
    }

    sqlx::query!("update web_media_upload set processed_at = now(), processing_result = true where media_id = $1", id).execute(&mut txn).await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn process_animation(
    db: &PgPool,
    s3: &crate::service::s3::Client,
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

pub async fn process_user_audio(
    db: &PgPool,
    s3: &service::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        //language=SQL
        r#"
select exists(select 1
from user_audio_library
inner join user_audio_upload on user_audio_library.id = user_audio_upload.audio_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of user_audio_upload
for share of user_audio_library
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
        .download_media_for_processing(MediaLibrary::User, id, FileKind::AudioMp3)
        .await?;

    // todo: use the duration
    // let _duration = {
    //     let bytes = bytes.clone();
    //     tokio::task::spawn_blocking(move || {
    //         mp3_metadata::read_from_slice(&bytes).map_err(|_it| error::Upload::InvalidMedia)
    //     })
    //         .await
    //         .unwrap()?
    // };

    let file = match file {
        Some(it) => it,
        None => {
            sqlx::query!("update user_audio_upload set processed_at = now(), processing_result = false where audio_id = $1", id)
                .execute(&mut txn)
                .await?;

            log::warn!("Audio wasn't uploaded properly before processing?");
            txn.commit().await?;
            return Ok(true);
        }
    };

    // todo: processing

    s3.upload_media(file, MediaLibrary::User, id, FileKind::AudioMp3)
        .await?;

    sqlx::query!("update user_audio_upload set processed_at = now(), processing_result = true where audio_id = $1", id).execute(&mut txn).await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn process_user_pdf(
    db: &PgPool,
    s3: &service::s3::Client,
    id: Uuid,
) -> anyhow::Result<bool> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        //language=SQL
        r#"
select exists(select 1
from user_pdf_library
inner join user_pdf_upload on user_pdf_library.id = user_pdf_upload.pdf_id
where (id = $1 and uploaded_at is not null and processed_at >= uploaded_at is not true)
for no key update of user_pdf_upload
for share of user_pdf_library
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
        .download_media_for_processing(MediaLibrary::User, id, FileKind::DocumentPdf)
        .await?;

    // todo: use the duration
    // let _duration = {
    //     let bytes = bytes.clone();
    //     tokio::task::spawn_blocking(move || {
    //         mp3_metadata::read_from_slice(&bytes).map_err(|_it| error::Upload::InvalidMedia)
    //     })
    //         .await
    //         .unwrap()?
    // };

    let file = match file {
        Some(it) => it,
        None => {
            sqlx::query!("update user_pdf_upload set processed_at = now(), processing_result = false where pdf_id = $1", id)
                .execute(&mut txn)
                .await?;

            log::warn!("Pdf wasn't uploaded properly before processing?");
            txn.commit().await?;
            return Ok(true);
        }
    };

    // todo: processing

    s3.upload_media(file, MediaLibrary::User, id, FileKind::DocumentPdf)
        .await?;

    sqlx::query!("update user_pdf_upload set processed_at = now(), processing_result = true where pdf_id = $1", id).execute(&mut txn).await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn finalize_upload(
    access_token: &str,
    notifications: &service::notifications::Client,
    library: &MediaLibrary,
    id: &Uuid,
) -> anyhow::Result<()> {
    notifications
        .signal_status_ready(access_token, library, id)
        .await?;
    Ok(())
}
