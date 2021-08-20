use crate::{db, error, extractor::TokenUser};

use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use shared::{
    api::endpoints::{user::GetColors, ApiEndpoint},
    domain::user::{UserColorResponse, UserColorValueRequest},
};
use sqlx::PgPool;

pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserColorValueRequest>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.0.user_id;

    let colors = db::user::create_color(db.as_ref(), user_id, req.into_inner().color).await?;
    Ok(HttpResponse::Created().json(UserColorResponse { colors }))
}

pub async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserColorValueRequest>,
    index: Path<u16>,
) -> Result<HttpResponse, error::NotFound> {
    let user_id = claims.0.user_id;

    let exists = db::user::update_color(
        db.as_ref(),
        user_id,
        index.into_inner(),
        req.into_inner().color,
    )
    .await?;

    if !exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<GetColors as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.0.user_id;

    let colors = db::user::get_colors(db.as_ref(), user_id).await?;
    Ok(Json(UserColorResponse { colors }))
}

pub async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    index: Path<u16>,
) -> Result<HttpResponse, error::Delete> {
    let user_id = claims.0.user_id;

    db::user::delete_color(db.as_ref(), user_id, index.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}
