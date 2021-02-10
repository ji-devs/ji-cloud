use crate::{
    db::{self, user::register},
    error,
};
use crate::{extractor::TokenUser, token::create_signin_token};
use actix_web::HttpResponse;
use core::settings::RuntimeSettings;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Query, ServiceConfig},
};
use shared::{
    api::endpoints::{
        user::{Profile, Register, UserLookup},
        ApiEndpoint,
    },
    domain::{
        auth::{RegisterRequest, RegisterSuccess},
        user::UserLookupQuery,
    },
    error::auth::RegisterErrorKind,
};
use sqlx::PgPool;

/// Lookup a user.
#[api_v2_operation]
async fn user_lookup(
    db: Data<PgPool>,
    query: Query<UserLookupQuery>,
) -> Result<Json<<UserLookup as ApiEndpoint>::Res>, error::UserNotFound> {
    let query = query.into_inner();

    db::user::lookup(db.as_ref(), query.id, query.name.as_deref())
        .await?
        .map(Json)
        .ok_or(error::UserNotFound::UserNotFound)
}

async fn validate_register_req(req: &RegisterRequest) -> Result<(), error::Register> {
    // todo: decide if we should check for an _empty_ email?
    if req.username.is_empty() {
        return Err(error::Register::RegisterError(
            RegisterErrorKind::EmptyDisplayName,
        ));
    }

    Ok(())
}

/// Register a new user.
#[allow(unreachable_code, unused_variables)]
#[api_v2_operation]
async fn handle_register(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    // user: FirebaseUser,
    req: Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, error::Register> {
    validate_register_req(&req).await?;

    let id = register(db.as_ref(), &req).await?;

    // fixme: remove the todo, remove the `#[allow]` above.
    let (csrf, cookie) = create_signin_token(
        id,
        &settings.token_secret,
        settings.is_local(),
        todo!("re-implement todo"),
    )?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(RegisterSuccess::Signin(csrf)))
}

/// Get a user by their profile.
#[api_v2_operation]
async fn handle_get_profile(
    db: Data<PgPool>,
    claims: TokenUser,
) -> Result<Json<<Profile as ApiEndpoint>::Res>, error::UserNotFound> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err`

    db::user::profile(db.as_ref(), claims.0.sub)
        .await?
        .map(Json)
        .ok_or(error::UserNotFound::UserNotFound)
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        Profile::PATH,
        Profile::METHOD.route().to(handle_get_profile),
    )
    .route(Register::PATH, Register::METHOD.route().to(handle_register))
    .route(UserLookup::PATH, UserLookup::METHOD.route().to(user_lookup));
}
