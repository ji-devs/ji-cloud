use crate::{db, error, extractor::TokenUser};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use chrono::{Duration, Utc};
use core::settings::RuntimeSettings;
use shared::{
    api::{endpoints::jig::player, ApiEndpoint},
    domain::jig::{
        player::{JigPlayCount, JigPlayerSession, JigPlayerSessionCode, JigPlayerSessionToken},
        JigId,
    },
};
use sqlx::PgPool;

use crate::extractor::IPAddress;
use crate::token::{create_auth_token_no_cookie, generate_csrf, validate_token};

/// Create a jig player session for the author, if one does not exist already.
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

/// Create a jig player session for someone who's not the author, if one doesn't already exist
pub async fn create_player_session(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    claims: TokenUser,
    ip_addr: IPAddress,
    req: Json<<player::CreatePlayerSession as ApiEndpoint>::Req>,
) -> Result<
    (
        Json<<player::CreatePlayerSession as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::JigCode,
> {
    let req = req.into_inner();

    let session_id = db::jig::player::create_user_session(&db, req.session_index, ip_addr).await?;

    // Generate a short-lived access token that will authenticate the next API
    let session_duration = Duration::weeks(1);
    let csrf = generate_csrf();
    let now = Utc::now();

    let token: String = create_auth_token_no_cookie(
        &settings.token_secret,
        session_duration,
        &session_id,
        csrf.clone(),
        now,
    )?;

    Ok((
        Json(JigPlayerSessionToken { token }),
        actix_web::http::StatusCode::CREATED, // FIXME this
    ))
}

/// Create a jig player session for someone who's not the author, if one doesn't already exist
pub async fn complete_player_session(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    claims: TokenUser,
    ip_addr: IPAddress,
    req: Json<<player::CompletePlayerSession as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::JigCode> {
    let req = req.into_inner();

    // todo make a token error
    let token = validate_token(&req.token, "authorized", &settings.token_secret)
        .expect("invalid player session token");

    let session_id = token.get("sub").unwrap().as_str().unwrap();

    db::jig::player::complete_user_session(&db, req.jig_id, ip_addr, &session_id).await?;

    Ok(HttpResponse::NoContent().finish())
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
