use crate::{
    db::{
        self,
        user::{register, register_google_oauth},
    },
    error,
    extractor::OAuthSignupToken,
    token::{OAuthProvider, TokenSource},
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
    domain::{auth::RegisterRequest, session::CreateSessionSuccess, user::UserLookupQuery},
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
    // todo: Option ignores errors, don't do that.
    signup_user: OAuthSignupToken,
    req: Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, error::Register> {
    let signup_user = Some(signup_user);
    match signup_user {
        None => return Err(anyhow::anyhow!("handle authless register request").into()),
        Some(token) => {
            validate_register_req(&req).await?;

            let mut txn = db.begin().await?;

            let id = register(&mut txn, &req, &token.0.email).await?;

            match &token.0.provider {
                OAuthProvider::Google { google_id } => {
                    register_google_oauth(&mut txn, &google_id, id).await?;
                }
            }

            txn.commit().await?;

            // fixme: remove the todo, remove the `#[allow]` above.
            let (csrf, cookie) = create_signin_token(
                id,
                &settings.token_secret,
                TokenSource::OAuth(token.0.provider),
                settings.login_token_valid_duration,
            )?;

            Ok(HttpResponse::Created()
                .cookie(cookie)
                .json(CreateSessionSuccess { csrf }))
        }
    }
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
