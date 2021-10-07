use crate::{db, error, extractor::TokenUser};
use actix_web::{
    http::StatusCode,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use shared::{
    api::{endpoints::image::recent, ApiEndpoint},
    domain::image::{
        recent::{UserRecentImageListResponse, UserRecentImageResponse},
        ImageId,
    },
    media::MediaLibrary,
};
use sqlx::PgPool;

pub(in super::super) async fn put(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<recent::Put as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::UserRecentImage> {
    // TODO: new: return created; updated: return Ok
    let (id, library, last_used, is_updated): (ImageId, MediaLibrary, DateTime<Utc>, bool) =
        db::image::recent::upsert(db.as_ref(), claims.0.user_id, req.id, req.library).await?;

    let status_code = if is_updated == true {
        StatusCode::OK
    } else {
        StatusCode::CREATED
    };

    Ok(
        HttpResponse::build(status_code).json(UserRecentImageResponse {
            id,
            library,
            last_used,
        }),
    )
}

pub(in super::super) async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Path<ImageId>,
) -> Result<HttpResponse, error::UserRecentImage> {
    db::image::recent::delete(db.as_ref(), claims.0.user_id, req.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub(in super::super) async fn list(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Option<Query<<recent::List as ApiEndpoint>::Req>>,
) -> Result<Json<<recent::List as ApiEndpoint>::Res>, error::UserRecentImage> {
    // Handle optional limit here.
    // `None` becomes `limit null` in postgres query. This means no limit.
    let limit = match req {
        None => None,
        Some(query) => {
            let limit = query.into_inner().limit;
            Some(limit as i64)
        }
    };

    let images: Vec<UserRecentImageResponse> =
        db::image::recent::list(db.as_ref(), claims.0.user_id, limit).await?;

    Ok(Json(UserRecentImageListResponse { images }))
}
