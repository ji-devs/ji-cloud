use crate::{db, error, extractor::TokenUser};

use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Path},
    CreatedJson, NoContent,
};
use shared::{
    api::endpoints::{
        user::{CreateFont, GetFonts},
        ApiEndpoint,
    },
    domain::user::{UserFontNameRequest, UserFontResponse},
};
use sqlx::PgPool;

#[api_v2_operation]
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserFontNameRequest>,
) -> Result<CreatedJson<<CreateFont as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.0.user_id;

    let fonts = db::user::create_font(db.as_ref(), user_id, req.into_inner().font_name).await?;
    Ok(CreatedJson(UserFontResponse { fonts }))
}

#[api_v2_operation]
pub async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserFontNameRequest>,
    index: Path<u16>,
) -> Result<NoContent, error::NotFound> {
    let user_id = claims.0.user_id;

    let exists =
        db::user::update_font(db.as_ref(), user_id, *index, req.into_inner().font_name).await?;

    if !exists {
        return Err(error::NotFound::ResourceNotFound);
    }

    Ok(NoContent)
}

#[api_v2_operation]
pub async fn get(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<GetFonts as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.0.user_id;

    let fonts = db::user::get_fonts(db.as_ref(), user_id).await?;
    Ok(Json(UserFontResponse { fonts }))
}

#[api_v2_operation]
pub async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    index: Path<u16>,
) -> Result<NoContent, error::Delete> {
    let user_id = claims.0.user_id;

    db::user::delete_font(db.as_ref(), user_id, *index).await?;

    Ok(NoContent)
}
