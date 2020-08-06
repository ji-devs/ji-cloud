use crate::db::{
    self,
    user::{profile_by_firebase, register},
};
use crate::extractor::{
    reply_signin_auth, FirebaseId, FirebaseUser, WrapAuthClaimsCookieDbNoCsrf, WrapAuthClaimsNoDb,
};
use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
};
use core::settings::Settings;
use jsonwebtoken as jwt;
use shared::{
    api::endpoints::{
        user::{Profile, Register, Signin, SingleSignOn},
        ApiEndpoint,
    },
    auth::{AuthClaims, RegisterError, RegisterRequest, SigninSuccess, SingleSignOnSuccess},
    user::NoSuchUserError,
};
use sqlx::PgPool;

async fn handle_signin_credentials(
    settings: Data<Settings>,
    db: Data<PgPool>,
    user: FirebaseUser,
) -> actix_web::Result<HttpResponse> {
    if !db::user::exists_by_firebase(&db, &user.id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
    {
        return Err(HttpResponse::UnprocessableEntity()
            .json(NoSuchUserError {})
            .into());
    }

    let (csrf, cookie) =
        reply_signin_auth(user.id, &settings.jwt_encoding_key, settings.local_insecure)
            .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(SigninSuccess { csrf }))
}

async fn validate_register_req(req: &RegisterRequest) -> Result<(), RegisterError> {
    // todo: decide if we should check for an _empty_ email?
    if req.display_name.is_empty() {
        return Err(RegisterError::EmptyDisplayName);
    }

    Ok(())
}

async fn handle_register(
    settings: Data<Settings>,
    db: Data<PgPool>,
    user: FirebaseUser,
    req: Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, RegisterError> {
    validate_register_req(&req).await?;

    register(db.as_ref(), &user.id, &req).await?;

    let (csrf, cookie) =
        reply_signin_auth(user.id, &settings.jwt_encoding_key, settings.local_insecure)
            .map_err(|_| RegisterError::InternalServerError)?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(SigninSuccess { csrf }))
}

async fn handle_get_profile(
    db: Data<PgPool>,
    claims: WrapAuthClaimsNoDb,
) -> actix_web::Result<Json<<Profile as ApiEndpoint>::Res>> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err

    profile_by_firebase(db.as_ref(), &FirebaseId(claims.0.id))
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .map(Json)
        .ok_or(HttpResponse::NotFound().json(NoSuchUserError {}).into())
}

async fn handle_authorize(
    settings: Data<Settings>,
    auth: WrapAuthClaimsCookieDbNoCsrf,
) -> actix_web::Result<Json<<SingleSignOn as ApiEndpoint>::Res>> {
    log::info!("Firebase is valid! user id is: {}", auth.0.id);

    let claims = AuthClaims {
        id: auth.0.id,
        csrf: None,
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, &settings.jwt_encoding_key)
        .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(Json(SingleSignOnSuccess { jwt }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <Profile as ApiEndpoint>::PATH,
        web::get().to(handle_get_profile),
    )
    .route(
        <SingleSignOn as ApiEndpoint>::PATH,
        web::post().to(handle_authorize),
    )
    .route(
        <Register as ApiEndpoint>::PATH,
        web::post().to(handle_register),
    )
    .route(
        <Signin as ApiEndpoint>::PATH,
        web::post().to(handle_signin_credentials),
    );
}
