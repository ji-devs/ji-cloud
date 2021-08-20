use crate::{db, error, extractor::TokenUser};
use actix_web::{
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

pub(in super::super) async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<recent::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::UserRecentImage> {
    let req = req.into_inner();

    let (id, library, last_used): (ImageId, MediaLibrary, DateTime<Utc>) =
        db::image::recent::create(db.as_ref(), claims.0.user_id, req.id, req.library).await?;

    Ok(HttpResponse::Created().json(UserRecentImageResponse {
        id,
        library,
        last_used,
    }))
}

pub(in super::super) async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Path<ImageId>,
) -> Result<HttpResponse, error::UserRecentImage> {
    db::image::recent::update(db.as_ref(), claims.0.user_id, req.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
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
