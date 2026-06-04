use crate::image_ops::MediaKind;
use crate::service::{s3, ServiceData};
use actix_web::{http::StatusCode, web::Bytes};
use anyhow::Context;
use ji_core::config::{ANIMATION_BODY_SIZE_LIMIT, IMAGE_BODY_SIZE_LIMIT};
use sha2::Digest;
use shared::{
    domain::image::ImageSize,
    media::{FileKind, MediaLibrary},
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[inline]
const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

pub async fn create(
    pool: &PgPool,
    s3: &ServiceData<s3::Client>,
    url_string: &String,
) -> anyhow::Result<(Uuid, MediaKind, StatusCode)> {
    // If we can already find the image, return early.
    if let Some(record) = sqlx::query!(
        r#"
select media_id,
       kind as "kind: MediaKind"
from web_media_library_url
inner join web_media_library on id = media_id
where media_url = $1"#,
        &url_string
    )
    .fetch_optional(pool)
    .await?
    {
        log::trace!("Found the url");

        return Ok((record.media_id, record.kind, StatusCode::OK));
    }

    let data: Vec<u8> = download_media_file(&url_string)
        .await
        .context("failed to download web media")?;

    let mut txn = pool.begin().await?;

    let (hash, id, kind) = hash_media_file(url_string.to_string(), &mut txn, &data).await?;

    if let (Some(id), Some(kind)) = (id, kind) {
        txn.commit().await?;

        return Ok((id, kind, StatusCode::OK));
    }

    // insert row for uploads

    let kind = actix_web::web::block({
        let data = data.clone();
        move || crate::image_ops::detect_image_kind(&data)
    })
    .await??;

    log::debug!("detected image kind as: {:?}", kind);

    let id = sqlx::query!(
        r#"insert into web_media_library ("hash", kind) values($1, $2) returning id"#,
        &hash,
        kind as i16
    )
    .fetch_one(&mut txn)
    .await?
    .id;

    sqlx::query!(
        "insert into web_media_library_url (media_id, media_url) values ($1, $2)",
        id,
        &url_string
    )
    .execute(&mut txn)
    .await?;

    process_web_media_bytes(s3, id, kind, data)
        .await
        .context("failed to process web media")?;

    sqlx::query!(
        "insert into web_media_upload (media_id, uploaded_at) values ($1, now())",
        id,
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        "update web_media_upload set processed_at = now(), processing_result = true where media_id = $1",
        id
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok((id, kind, StatusCode::CREATED))
}

async fn process_web_media_bytes(
    s3: &s3::Client,
    id: Uuid,
    kind: MediaKind,
    data: Vec<u8>,
) -> anyhow::Result<()> {
    match kind {
        MediaKind::GifAnimation => {
            s3.upload_media(data, MediaLibrary::Web, id, FileKind::AnimationGif)
                .await?;
        }
        MediaKind::PngStickerImage => {
            let (original, resized, thumbnail) = actix_web::web::block(move || {
                let original = image::load_from_memory(&data)?;
                crate::image_ops::generate_images(&original, ImageSize::Sticker)
            })
            .await??;

            s3.upload_png_images(MediaLibrary::Web, id, original, resized, thumbnail)
                .await?;
        }
        kind => return Err(anyhow::anyhow!("unsupported media kind {:?}", kind)),
    }

    Ok(())
}

async fn download_media_file(url_string: &str) -> anyhow::Result<Vec<u8>> {
    const MAX_RESPONSE_SIZE: usize = max(ANIMATION_BODY_SIZE_LIMIT, IMAGE_BODY_SIZE_LIMIT);

    // Download media file
    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // todo: this `?` should be a ClientError or "proxy/gateway error"
    let mut response: reqwest::Response = client.get(url_string).send().await?;

    let mut data = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        let chunk: Bytes = chunk;
        if data.len() + chunk.len() < MAX_RESPONSE_SIZE {
            data.extend_from_slice(&chunk[..]);
        } else {
            return Err(anyhow::anyhow!("todo: better error here (data too big)").into());
        }
    }

    log::trace!("data was {} bytes long", data.len());

    Ok(data)
}

async fn hash_media_file(
    url_string: String,
    txn: &mut PgConnection,
    data: &Vec<u8>,
) -> sqlx::Result<(Vec<u8>, Option<Uuid>, Option<MediaKind>)> {
    let mut hasher = sha2::Sha384::new();

    hasher.update(&data);

    let hash = hasher.finalize().to_vec();

    // If we can find the image by hash, return early.

    let record = sqlx::query!(
        r#"
select id,
       hash,
       kind as "kind: MediaKind"
from web_media_library
where hash = $1
for update
"#,
        &hash
    )
    .fetch_optional(&mut *txn)
    .await?;

    if let Some(record) = record {
        let id = record.id;
        sqlx::query!(
            "insert into web_media_library_url (media_id, media_url) values ($1, $2) on conflict (media_id, media_url) do nothing",
            id,
            &url_string
        )
        .execute(&mut *txn)
        .await?;

        log::trace!("Found the hash");

        return Ok((hash, Some(record.id), Some(record.kind)));
    }

    Ok((hash, None, None))
}
