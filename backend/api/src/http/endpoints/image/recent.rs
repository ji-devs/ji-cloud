use crate::{db, error, extractor::TokenUser};
use chrono::{DateTime, Utc};
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Path, Query},
    CreatedJson, NoContent,
};
use shared::{
    api::{endpoints::image::recent, ApiEndpoint},
    domain::image::{
        recent::{UserRecentImageListResponse, UserRecentImageResponse},
        ImageId,
    },
    media::MediaLibrary,
};
use sqlx::PgPool;

#[api_v2_operation]
pub(in super::super) async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<recent::Create as ApiEndpoint>::Req>,
) -> Result<CreatedJson<UserRecentImageResponse>, error::UserRecentImage> {
    let req = req.into_inner();

    let (id, library, last_used): (ImageId, MediaLibrary, DateTime<Utc>) =
        db::image::recent::create(db.as_ref(), claims.0.user_id, req.id, req.library).await?;

    Ok(CreatedJson(UserRecentImageResponse {
        id,
        library,
        last_used,
    }))
}

#[api_v2_operation]
pub(in super::super) async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Path<ImageId>,
) -> Result<NoContent, error::UserRecentImage> {
    db::image::recent::update(db.as_ref(), claims.0.user_id, req.into_inner()).await?;

    Ok(NoContent)
}

#[api_v2_operation]
pub(in super::super) async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Path<ImageId>,
) -> Result<NoContent, error::UserRecentImage> {
    db::image::recent::delete(db.as_ref(), claims.0.user_id, req.into_inner()).await?;

    Ok(NoContent)
}

#[api_v2_operation]
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
