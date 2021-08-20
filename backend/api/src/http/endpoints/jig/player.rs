use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};
use shared::{
    api::{endpoints::jig::player, ApiEndpoint},
    domain::jig::{
        player::{JigPlayerSession, JigPlayerSessionCode},
        JigId,
    },
};

/// Create a jig player session, if one does not exist already.
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<player::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::JigCode> {
    let req = req.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(req.jig_id.clone())).await?;

    let index = db::jig::player::create(&db, req.jig_id, req.settings).await?;

    Ok(HttpResponse::Created().json(JigPlayerSessionCode { index }))
}

/// Get the player session identified by the code, if it exists.
pub async fn get(
    db: Data<PgPool>,
    path: web::Path<i16>,
) -> Result<Json<<player::Get as ApiEndpoint>::Res>, error::JigCode> {
    let code = path.into_inner();

    let res = db::jig::player::get(&*db, code)
        .await?
        .ok_or(error::JigCode::ResourceNotFound)?;

    Ok(Json(JigPlayerSession {
        jig_id: res.0,
        settings: res.1,
    }))
}

/// Fetch a jig player session code from it's jig if it exists.
pub async fn get_code(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<player::GetPlayerSessionCode as ApiEndpoint>::Res>, error::JigCode> {
    let id = path.into_inner();

    let index = db::jig::player::get_code(&*db, id)
        .await?
        .ok_or(error::JigCode::ResourceNotFound)?;

    Ok(Json(JigPlayerSessionCode { index }))
}
