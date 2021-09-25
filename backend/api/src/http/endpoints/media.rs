use actix_web::{
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, HttpResponseBuilder,
};
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        media::{UrlCreatedResponse, WebMediaMetadataResponse, WebMediaUrlCreateRequest},
        Base64,
    },
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUser, TokenUserWithScope},
    image_ops::MediaKind,
    service::{s3, ServiceData},
};

pub async fn create(
    pool: Data<PgPool>,
    _claims: TokenUser,
    s3: ServiceData<s3::Client>,
    request: Json<WebMediaUrlCreateRequest>,
) -> Result<HttpResponse, error::Server> {
    let request = request.into_inner();

    let url_string = request.url.to_string();

    let (id, kind, status_code) = db::media::create(&pool, &s3, &url_string).await?;

    Ok(
        HttpResponseBuilder::new(status_code).json(UrlCreatedResponse {
            id,
            kind: kind.to_shared(),
        }),
    )
}

// filter by media type, etc
// list route (with filter & pagation)
// async fn list() {}

async fn delete_media(
    pool: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    s3: ServiceData<s3::Client>,
    path: Path<Uuid>,
) -> Result<HttpResponse, error::Server> {
    let id = path.into_inner();

    let record = sqlx::query!(
        r#"delete from web_media_library where id = $1 returning kind as "kind: MediaKind""#,
        id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let kind = match record {
        Some(record) => record.kind,
        None => return Ok(HttpResponse::NoContent().finish()),
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

    Ok(HttpResponse::NoContent().finish())
}

async fn delete_url(
    pool: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    url: Path<Base64<Url>>,
) -> Result<HttpResponse, error::Server> {
    let url = url.into_inner().0;
    sqlx::query!(
        "delete from web_media_library_url where media_url = $1",
        url.to_string()
    )
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

async fn get(
    pool: Data<PgPool>,
    _claims: TokenUser,
    path: Path<Uuid>,
) -> Result<Json<WebMediaMetadataResponse>, error::NotFound> {
    let id = path.into_inner();

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

async fn get_by_url(
    pool: Data<PgPool>,
    _claims: TokenUser,
    path: Path<Base64<Url>>, // FIXME
) -> Result<Json<WebMediaMetadataResponse>, error::NotFound> {
    let Base64(url) = path.into_inner();

    // let url = Base64::decode()

    let media = sqlx::query!(
        //language=SQL
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

pub fn configure(cfg: &mut ServiceConfig) {
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
