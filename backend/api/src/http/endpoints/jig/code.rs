use paperclip::actix::{
    api_v2_operation,
    web::{self, Data, Json},
    CreatedJson,
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};
use shared::domain::jig::code::JigIdFromCodeResponse;
use shared::{
    api::{endpoints::jig::code, ApiEndpoint},
    domain::jig::{code::JigCodeResponse, JigId},
};

/// Create a jig code, if one does not exist already.
#[api_v2_operation]
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<CreatedJson<<code::Create as ApiEndpoint>::Res>, error::JigCode> {
    let id = path.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(id)).await?;

    let code = db::jig::code::create(&db, id).await?;

    Ok(CreatedJson(JigCodeResponse { code }))
}

/// Fetch a jig code, if it exists. Does not require
#[api_v2_operation]
pub async fn get(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<code::Get as ApiEndpoint>::Res>, error::JigCode> {
    let id = path.into_inner();

    let code = db::jig::code::get(&*db, id)
        .await?
        .ok_or(error::JigCode::ResourceNotFound)?;

    Ok(Json(JigCodeResponse { code }))
}

/// Get the jig associated with the code, if it exists.
#[api_v2_operation]
pub async fn get_jig_from_code(
    db: Data<PgPool>,
    _claims: TokenUser,
    req: Json<<code::GetJig as ApiEndpoint>::Req>,
) -> Result<Json<<code::GetJig as ApiEndpoint>::Res>, error::JigCode> {
    let code = req.into_inner().code;

    let id = db::jig::code::get_jig_from_code(&*db, code)
        .await?
        .ok_or(error::JigCode::ResourceNotFound)?;

    Ok(Json(JigIdFromCodeResponse { id }))
}
