use std::sync::Arc;

use crate::{
    error,
    extractor::{ScopeAdmin, TokenUser, TokenUserWithScope},
    image_ops::MediaKind,
    s3,
    service::ServiceData,
};
use actix_web::web::Path;
use paperclip::actix::{
    api_v2_operation,
    web::{Bytes, Data, Json, ServiceConfig},
    CreatedJson, NoContent,
};
use sha2::Digest as _;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::media::{UrlCreatedResponse, WebMediaMetadataResponse, WebMediaUrlCreateRequest},
    media::{FileKind, PngImageFile},
};
use shared::{
    domain::{image::ImageKind, Base64},
    media::MediaLibrary,
};
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

#[api_v2_operation]
pub async fn create(
    pool: Data<PgPool>,
    _claims: TokenUser,
    s3: ServiceData<s3::Client>,
    request: Json<WebMediaUrlCreateRequest>,
) -> Result<CreatedJson<UrlCreatedResponse>, error::Server> {
    let url = request.into_inner().url;

    const MAX_RESPONSE_SIZE: usize = max(
        config::ANIMATION_BODY_SIZE_LIMIT,
        config::IMAGE_BODY_SIZE_LIMIT,
    );

    let url_string = url.to_string();

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
    .fetch_optional(pool.as_ref())
    .await?
    {
        log::trace!("Found the url");

        return Ok(CreatedJson(UrlCreatedResponse {
            id: record.media_id,
            kind: record.kind.to_shared(),
        }));
    }

    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // todo: this `?` should be a ClientError or "proxy/gateway error"
    let mut response: reqwest::Response = client.get(url).send().await?;

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

    let mut hasher = sha2::Sha384::new();

    hasher.update(&data);

    let hash = hasher.finalize().to_vec();

    let mut txn = pool.begin().await?;

    // If we can find the image by hash, return early.

    let record = sqlx::query!(
        r#"
select id,
       kind as "kind: MediaKind"
from web_media_library
where hash = $1
for update
"#,
        &hash
    )
    .fetch_optional(&mut txn)
    .await?;

    if let Some(record) = record {
        let id = record.id;
        sqlx::query!(
            "insert into web_media_library_url (media_id, media_url) values ($1, $2) on conflict (media_id, media_url) do nothing",
            id,
            &url_string
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        log::trace!("Found the hash");

        return Ok(CreatedJson(UrlCreatedResponse {
            id,
            kind: record.kind.to_shared(),
        }));
    }

    let data = Arc::new(data);

    let kind = actix_web::web::block({
        let data = data.clone();
        move || crate::image_ops::detect_image_kind(&data)
    })
    .await?;

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

    match kind {
        MediaKind::GifAnimation => {
            s3.upload_media(
                Arc::try_unwrap(data).expect("This should be unique by now"),
                MediaLibrary::Web,
                id,
                FileKind::AnimationGif,
            )
            .await?;
        }

        MediaKind::PngStickerImage => {
            let (original, resized, thumbnail) = actix_web::web::block(move || {
                let original = image::load_from_memory(&data)?;
                crate::image_ops::generate_images(&original, ImageKind::Sticker)
            })
            .await?;

            s3.upload_png_images(MediaLibrary::Web, id, original, resized, thumbnail)
                .await?;
        }

        kind => return Err(anyhow::anyhow!("unsupported media kind {:?}", kind).into()),
    }

    sqlx::query!(
        "update web_media_library set uploaded_at = now() where id = $1",
        id
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(CreatedJson(UrlCreatedResponse {
        id,
        kind: kind.to_shared(),
    }))
}

// filter by media type, etc
// list route (with filter & pagation)
// async fn list() {}

#[api_v2_operation]
async fn delete_media(
    pool: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    s3: ServiceData<s3::Client>,
    Path(id): Path<Uuid>,
) -> Result<NoContent, error::Server> {
    let record = sqlx::query!(
        r#"delete from web_media_library where id = $1 returning kind as "kind: MediaKind""#,
        id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let kind = match record {
        Some(record) => record.kind,
        None => return Ok(NoContent),
    };

    let delete = |file_kind| s3.delete_media(MediaLibrary::Web, file_kind, id);
    match kind {
        MediaKind::PngStickerImage => {
            futures::future::join3(
                delete(FileKind::ImagePng(PngImageFile::Original)),
                delete(FileKind::ImagePng(PngImageFile::Resized)),
                delete(FileKind::ImagePng(PngImageFile::Thumbnail)),
            )
            .await;
        }

        MediaKind::GifAnimation => {
            delete(FileKind::AnimationGif).await;
        }

        kind => return Err(anyhow::anyhow!("unsupported media kind {:?}", kind).into()),
    }

    Ok(NoContent)
}

#[api_v2_operation]
async fn delete_url(
    pool: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    url: Path<Base64<Url>>,
) -> Result<NoContent, error::Server> {
    let url = url.into_inner().0;
    sqlx::query!(
        "delete from web_media_library_url where media_url = $1",
        url.to_string()
    )
    .execute(pool.as_ref())
    .await?;

    Ok(NoContent)
}

#[api_v2_operation]
async fn get(
    pool: Data<PgPool>,
    _claims: TokenUser,
    Path(id): Path<Uuid>,
) -> Result<Json<WebMediaMetadataResponse>, error::NotFound> {
    let media = sqlx::query!(
        r#"
select id,
       kind as "kind: MediaKind",
       created_at,
       updated_at,
       array(select media_url from web_media_library_url where media_id = $1) as "urls!"
from web_media_library
where id = $1"#,
        id
    )
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(WebMediaMetadataResponse {
        id: media.id,
        kind: media.kind.to_shared(),
        urls: media
            .urls
            .into_iter()
            .map(|it| Url::parse(&it))
            .collect::<Result<Vec<_>, _>>()?,
        created_at: media.created_at,
        updated_at: media.updated_at,
    }))
}

#[api_v2_operation]
async fn get_by_url(
    pool: Data<PgPool>,
    _claims: TokenUser,
    Path(Base64(url)): Path<Base64<Url>>,
) -> Result<Json<WebMediaMetadataResponse>, error::NotFound> {
    let media = sqlx::query!(
        r#"
select id,
       kind as "kind: MediaKind",
       created_at,
       updated_at,
       array(select media_url from web_media_library_url where media_id = id) as "urls!"
from web_media_library
where id = (select media_id from web_media_library_url where media_url = $1)
"#,
        url.to_string()
    )
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(WebMediaMetadataResponse {
        id: media.id,
        kind: media.kind.to_shared(),
        urls: media
            .urls
            .into_iter()
            .map(|it| Url::parse(&it))
            .collect::<Result<Vec<_>, _>>()?,
        created_at: media.created_at,
        updated_at: media.updated_at,
    }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        endpoints::media::Create::PATH,
        endpoints::media::Create::METHOD.route().to(create),
    )
    .route(
        endpoints::media::GetUrl::PATH,
        endpoints::media::GetUrl::METHOD.route().to(get_by_url),
    )
    .route(
        endpoints::media::GetId::PATH,
        endpoints::media::GetId::METHOD.route().to(get),
    )
    .route(
        endpoints::media::DeleteId::PATH,
        endpoints::media::DeleteId::METHOD.route().to(delete_url),
    )
    .route(
        endpoints::media::DeleteUrl::PATH,
        endpoints::media::DeleteUrl::METHOD.route().to(delete_media),
    );
}
