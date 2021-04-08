use crate::{db, error, extractor::TokenUser};

use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Path},
    CreatedJson, NoContent,
};
use shared::{
    api::endpoints::{
        user::{CreateColor, GetColors},
        ApiEndpoint,
    },
    domain::user::{UserColorResponse, UserColorValueRequest},
};
use sqlx::PgPool;

#[api_v2_operation]
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserColorValueRequest>,
) -> Result<CreatedJson<<CreateColor as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.0.user_id;

    let colors = db::user::create_color(db.as_ref(), user_id, req.into_inner().color).await?;
    Ok(CreatedJson(UserColorResponse { colors }))
}

#[api_v2_operation]
pub async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<UserColorValueRequest>,
    index: Path<u16>,
) -> Result<NoContent, error::NotFound> {
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

    Ok(NoContent)
}

#[api_v2_operation]
pub async fn get(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<GetColors as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.0.user_id;

    let colors = db::user::get_colors(db.as_ref(), user_id).await?;
    Ok(Json(UserColorResponse { colors }))
}

#[api_v2_operation]
pub async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    index: Path<u16>,
) -> Result<NoContent, error::Delete> {
    let user_id = claims.0.user_id;

    db::user::delete_color(db.as_ref(), user_id, index.into_inner()).await?;

    Ok(NoContent)
}
