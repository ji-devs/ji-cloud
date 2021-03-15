use core::settings::RuntimeSettings;

use actix_http::{
    error::BlockingError,
    http::header::{self, EntityTag, Header, IfMatch, IfNoneMatch},
};
use actix_web::{web::Json, HttpResponse};
use chrono::{DateTime, Duration, Utc};
use futures::TryStreamExt;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, HttpRequest, Path, ServiceConfig},
    NoContent,
};
use shared::{
    api::{endpoints::admin, ApiEndpoint},
    domain::{
        admin::{AdminListMediaResponse, AdminMediaItem},
        image::ImageKind,
        session::NewSessionResponse,
    },
    media::{FileKind, MediaLibrary, PngImageFile},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db, error,
    extractor::{ScopeAdmin, TokenUserWithScope},
    image_ops::{regenerate_images, MediaKind},
    s3,
    service::ServiceData,
    token::{create_auth_token, SessionMask},
};

/// Impersonate another user
#[api_v2_operation]
async fn impersonate(
    auth: TokenUserWithScope<ScopeAdmin>,
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    user: Path<Uuid>,
) -> actix_web::Result<HttpResponse, error::UserNotFound> {
    let user_id = user.into_inner();

    let exists = db::user::exists(&db, user_id).await?;

    if !exists {
        return Err(error::UserNotFound::UserNotFound);
    }

    let login_ttl = settings
        .login_token_valid_duration
        .unwrap_or(Duration::weeks(2));

    let session = db::session::create(
        &mut *db.acquire().await?,
        user_id,
        Some(&(Utc::now() + login_ttl)),
        SessionMask::GENERAL_API,
        Some(auth.claims.user_id),
    )
    .await?;

    let (csrf, cookie) = create_auth_token(
        &settings.token_secret,
        settings.is_local(),
        login_ttl,
        &session,
    )?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(NewSessionResponse { csrf }))
}

/// Forcefully refresh an item of media (as if it was just uploaded)
/// Note: this request can be conditional on `If-Match`
#[api_v2_operation]
async fn refresh_image_files(
    _auth: TokenUserWithScope<ScopeAdmin>,
    s3: ServiceData<s3::Client>,
    db: Data<PgPool>,
    Path((library, id)): Path<(MediaLibrary, Uuid)>,
    req: HttpRequest,
) -> actix_web::Result<NoContent, error::Refresh> {
    let if_match = IfMatch::parse(&req)
        .ok()
        .filter(|_| req.headers().contains_key(header::IF_MATCH));

    let if_none_match = IfNoneMatch::parse(&req)
        .ok()
        .filter(|_| req.headers().contains_key(header::IF_NONE_MATCH));

    let mut txn = db.begin().await?;

    let uploaded_at: Option<(Option<DateTime<Utc>>, ImageKind)> = match library {
        MediaLibrary::Web => sqlx::query!(
            "select uploaded_at from web_media_library where kind = $1 and id = $2 for update",
            crate::image_ops::MediaKind::PngStickerImage as _,
            id
        )
        .fetch_optional(&mut txn)
        .await?
        .map(|it| (it.uploaded_at, ImageKind::Sticker)),

        MediaLibrary::User => sqlx::query!(
            "select uploaded_at from user_image_upload where image_id = $1 for update",
            id
        )
        .fetch_optional(&mut txn)
        .await?
        .map(|it| (it.uploaded_at, ImageKind::Sticker)),

        MediaLibrary::Global => sqlx::query!(
            r#"select uploaded_at from image_upload where image_id = $1 for update"#,
            id
        )
        .fetch_optional(&mut txn)
        .await?
        .map(|it| (it.uploaded_at, ImageKind::Sticker)),
    };

    let (uploaded_at, kind): (Option<DateTime<Utc>>, ImageKind) =
        uploaded_at.ok_or(error::Refresh::ResourceNotFound)?;

    // Check if the media has already been updated.
    // If so, return `precondition failed`
    match if_match {
        Some(IfMatch::Items(items)) => {
            let uploaded_at = match uploaded_at {
                Some(uploaded_at) => EntityTag::strong(uploaded_at.timestamp_nanos().to_string()),
                None => {
                    return Err(error::Refresh::PreconditionFailed);
                }
            };

            if !items.iter().any(|item| item.strong_eq(&uploaded_at)) {
                return Err(error::Refresh::PreconditionFailed);
            }

            // we good here.
        }

        Some(IfMatch::Any) if !uploaded_at.is_some() => {
            return Err(error::Refresh::PreconditionFailed);
        }

        _ => {}
    }

    match if_none_match {
        Some(IfNoneMatch::Items(items)) => {
            if let Some(uploaded_at) = uploaded_at {
                let uploaded_at = EntityTag::strong(uploaded_at.timestamp_nanos().to_string());
                if items.iter().any(|item| item.strong_eq(&uploaded_at)) {
                    return Err(error::Refresh::PreconditionFailed);
                }
            }

            // we good here.
        }

        Some(IfNoneMatch::Any) if uploaded_at.is_some() => {
            return Err(error::Refresh::PreconditionFailed);
        }

        _ => {}
    }

    // handle the 404 here (image isn't uploaded)
    let original = s3
        .download_media_file(library, id, FileKind::ImagePng(PngImageFile::Original))
        .await?
        .ok_or(error::Refresh::ResourceNotFound)?;

    match library {
        MediaLibrary::Global => {
            sqlx::query!("update image_upload set uploaded_at = now(), processing_result = null where image_id = $1", id)
            .execute(&mut txn)
            .await?;

            return Ok(NoContent);
        }

        MediaLibrary::User => {
            sqlx::query!("update user_image_upload set uploaded_at = now(), processing_result = null where image_id = $1", id)
            .execute(&mut txn)
            .await?;

            return Ok(NoContent);
        }

        _ => {}
    }

    let (resized, thumbnail) = actix_web::web::block(move || -> Result<_, error::Refresh> {
        let original = image::load_from_memory(&original)?;

        Ok(regenerate_images(&original, kind)?)
    })
    .await
    .map_err(|err| match err {
        BlockingError::Canceled => anyhow::anyhow!("Thread pool is gone").into(),
        BlockingError::Error(e) => e,
    })?;

    s3.upload_png_images_resized_thumb(library, id, resized, thumbnail)
        .await?;

    match library {
            MediaLibrary::Web => sqlx::query!(
                "update web_media_library set uploaded_at = now(), updated_at = now() where kind = $1 and id = $2",
                crate::image_ops::MediaKind::PngStickerImage as _,
                id
            )
            .execute(&mut txn)
            .await?,

            MediaLibrary::User | MediaLibrary::Global => unreachable!(),
        };

    txn.commit().await?;

    Ok(NoContent)
}

#[api_v2_operation]
async fn list_media(
    _auth: TokenUserWithScope<ScopeAdmin>,
    db: Data<PgPool>,
) -> actix_web::Result<Json<AdminListMediaResponse>, error::Server> {
    let items: Vec<AdminMediaItem> = sqlx::query_file!("query/list_media.sql")
        .fetch(db.as_ref())
        .map_ok(|row| AdminMediaItem {
            id: row.id,
            kind: row.kind.to_shared(),
            created_at: row.created_at,
            updated_at: row.updated_at,
            uploaded_at: row.uploaded_at.clone(),
            file_etag: row.uploaded_at.map(|uploaded_at| {
                EntityTag::strong(uploaded_at.timestamp_nanos().to_string()).to_string()
            }),
            library: row.library,
        })
        .try_collect()
        .await?;

    items.len();

    items
        .iter()
        .filter(|it| matches!(it.kind, shared::media::MediaKind::Image(ImageKind::Canvas)))
        .count();

    Ok(Json(AdminListMediaResponse { media: items }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        admin::Impersonate::PATH,
        admin::Impersonate::METHOD.route().to(impersonate),
    )
    .route(
        admin::RefreshFiles::PATH,
        admin::RefreshFiles::METHOD.route().to(refresh_image_files),
    )
    .route(
        admin::ListMedia::PATH,
        admin::ListMedia::METHOD.route().to(list_media),
    );
}
