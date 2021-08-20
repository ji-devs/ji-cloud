use crate::{db, error, extractor::TokenUser};

use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use shared::{
    api::endpoints::{user, ApiEndpoint},
    domain::user::{UserFontNameRequest, UserFontResponse},
};
use sqlx::PgPool;

pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserFontNameRequest>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.0.user_id;

    let names = db::user::create_font(db.as_ref(), user_id, req.into_inner().name).await?;
    Ok(HttpResponse::Created().json(UserFontResponse { names }))
}

pub async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserFontNameRequest>,
    index: Path<u16>,
) -> Result<HttpResponse, error::NotFound> {
    let user_id = claims.0.user_id;

    let exists = db::user::update_font(db.as_ref(), user_id, *index, req.into_inner().name).await?;

    if !exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<user::GetFonts as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.0.user_id;

    let names = db::user::get_fonts(db.as_ref(), user_id).await?;
    Ok(Json(UserFontResponse { names }))
}

pub async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    index: Path<u16>,
) -> Result<HttpResponse, error::Delete> {
    let user_id = claims.0.user_id;

    db::user::delete_font(db.as_ref(), user_id, *index).await?;

    Ok(HttpResponse::NoContent().finish())
}
