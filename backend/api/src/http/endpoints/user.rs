use crate::db::user::{get_by_email, get_by_id, register};
use crate::extractor::{
    reply_signin_auth, FirebaseUser, WrapAuthClaimsCookieDbNoCsrf, WrapAuthClaimsNoDb,
};
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
    HttpResponse,
};
use core::settings::SETTINGS;
use jsonwebtoken as jwt;
use shared::{
    api::endpoints::{
        user::{Profile, SingleSignOn},
        ApiEndpoint,
    },
    auth::{AuthClaims, RegisterError, RegisterRequest, SingleSignOnSuccess},
    user::NoSuchUserError,
};
use sqlx::PgPool;

#[post("/user/signin")]
async fn handle_signin_credentials(
    db: Data<PgPool>,
    user: FirebaseUser,
) -> actix_web::Result<HttpResponse> {
    log::info!("Firebase is valid! user id is: {}", user.id);

    match get_by_id(db.as_ref(), &user.id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
    {
        Some(user) => reply_signin_auth(user.id, user.roles, false),
        None => {
            log::info!("hmm couldn't get user by id {}", user.id);

            Err(HttpResponse::UnprocessableEntity()
                .json(NoSuchUserError {})
                .into())
        }
    }
}

async fn validate_register_req(
    user_id: &str,
    db: &PgPool,
    req: &RegisterRequest,
) -> actix_web::Result<()> {
    let e = |err| Err(HttpResponse::UnprocessableEntity().json(err).into());

    if get_by_id(db, &user_id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .is_some()
    {
        return e(RegisterError::TakenId);
    }

    if get_by_email(db, &req.email)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .is_some()
    {
        return e(RegisterError::TakenEmail);
    }

    if req.display_name.is_empty() {
        return e(RegisterError::EmptyDisplayname);
    }

    if req.first_name.is_empty() {
        return e(RegisterError::EmptyFirstname);
    }

    if req.first_name.is_empty() {
        return e(RegisterError::EmptyLastname);
    }

    Ok(())
}

//register handler doesn't use the usual wrapper since it needs to set the header
#[post("/user/register")]
async fn handle_register(
    db: Data<PgPool>,
    user: FirebaseUser,
    req: Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse> {
    validate_register_req(&user.id, &db, &req).await?;

    register(db.as_ref(), &user.id, &req)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    reply_signin_auth(user.id, Vec::new(), true)
}

#[get("/user/profile")]
async fn handle_get_profile(
    db: Data<PgPool>,
    claims: WrapAuthClaimsNoDb,
) -> actix_web::Result<Json<<Profile as ApiEndpoint>::Res>> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err

    get_by_id(db.as_ref(), &claims.0.id)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .map(Json)
        .ok_or(HttpResponse::NotFound().json(NoSuchUserError {}).into())
}

#[post("/v1/authorize")]
async fn handle_get_sso_jwt(
    auth: WrapAuthClaimsCookieDbNoCsrf,
) -> actix_web::Result<Json<<SingleSignOn as ApiEndpoint>::Res>> {
    log::info!("Firebase is valid! user id is: {}", auth.0.id);

    let claims = AuthClaims {
        id: auth.0.id,
        csrf: None,
        roles: auth.0.roles,
    };

    let jwt = jwt::encode(
        &jwt::Header::default(),
        &claims,
        &SETTINGS.get().unwrap().jwt_encoding_key,
    )
    .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(Json(SingleSignOnSuccess { jwt }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(handle_get_profile)
        .service(handle_get_sso_jwt)
        .service(handle_register)
        .service(handle_signin_credentials);
}
