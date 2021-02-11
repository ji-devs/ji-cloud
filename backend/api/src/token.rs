use actix_http::cookie::{Cookie, CookieBuilder, SameSite};
use chrono::{Duration, Utc};
use config::{COOKIE_DOMAIN, MAX_SIGNIN_COOKIE_DURATION, MAX_SIGNUP_COOKIE_DURATION};
use http::StatusCode;
use paseto::{PasetoBuilder, TimeBackend};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use shared::domain::auth::AUTH_COOKIE_NAME;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{self, BasicError};

const AUTHORIZED_FOOTER: &str = "authorized";
const OAUTH_SIGNUP_FOOTER: &str = "oauth_signup";

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct OAuthSignupClaims {
    /// The email getting signed for
    #[serde(rename = "sub")]
    pub email: String,

    /// The csrf that must match for the token to be considered valid.
    csrf: String,

    /// What OAuth provider is being used
    pub provider: OAuthProvider,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The claims that are used as part of the user's token.
pub struct AuthorizedTokenClaims {
    /// The user claimed by the token.
    pub sub: Uuid,

    /// The csrf that must match for the token to be considered valid.
    csrf: String,

    /// What initially created this token (todo: replace with a session key or smth)
    source: TokenSource,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum OAuthProvider {
    Google { google_id: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum TokenSource {
    Basic,
    Impersonate(Uuid),
    OAuth(OAuthProvider),
}

fn validate_token(
    token_string: &str,
    footer: &str,
    token_key: &[u8; 32],
) -> Result<serde_json::Value, actix_web::Error> {
    let token =
        paseto::validate_local_token(token_string, Some(footer), token_key, &TimeBackend::Chrono);

    let err = match token {
        Ok(token) => return Ok(token),
        Err(e) => e,
    };

    let err_401 = |message: &'static str| {
        BasicError::with_message(StatusCode::UNAUTHORIZED, message.to_owned())
    };

    let e = match err.downcast::<paseto::errors::GenericError>() {
        Ok(paseto::errors::GenericError::ExpiredToken {}) => err_401("Expired token"),
        Ok(paseto::errors::GenericError::InvalidNotBeforeToken {}) => {
            err_401("Token is not valid yet")
        }
        Ok(paseto::errors::GenericError::InvalidFooter {}) => err_401(
            "Token footer is wrong (this currently means the token is meant for something else)",
        ),

        Ok(_) => err_401("Invalid token"),
        Err(e) => {
            return Err(
                error::ise(anyhow::anyhow!("Server failure for decoding token: {}", e)).into(),
            )
        }
    };

    Err(e.into())
}

// todo: accept a transaction instead so that we can do `for share` row locks
pub async fn check_login_token(
    db: &PgPool,
    token_string: &str,
    csrf: &str,
    token_key: &[u8; 32],
) -> Result<AuthorizedTokenClaims, actix_web::Error> {
    let token = validate_token(token_string, AUTHORIZED_FOOTER, token_key)?;

    let claims: AuthorizedTokenClaims = serde_json::from_value(token)
        .map_err(Into::into)
        .map_err(error::ise)?;

    if claims.csrf != csrf {
        return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
    }

    // todo: have some kind of session storage, etc, change this to a session instead/in addition to a source.
    match &claims.source {
        TokenSource::Basic => {
            let exists = sqlx::query!(
                r#"select exists(select 1 from user_auth_basic where user_id = $1) as "exists!""#,
                claims.sub,
            )
            .fetch_one(db)
            .await
            .map_err(Into::into)
            .map_err(error::ise)?;

            if !exists.exists {
                return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
            }
        }

        // Ensure the impersonator is still a admin (and exists), and that the impersonated user still exists.
        // todo: decide if admins can impersonate each other (and if this can be nested) - probably not.
        TokenSource::Impersonate(impersonator_id) => {
            let user_checks = sqlx::query!(
                r#"
select 
    exists(select 1 from "user" where id = $1) as "impersonated_exists!",
    exists(select 1 from user_scope where user_id = $2) as "impersonator_admin!"
"#,
                claims.sub,
                impersonator_id
            )
            .fetch_one(db)
            .await
            .map_err(Into::into)
            .map_err(error::ise)?;

            if !user_checks.impersonated_exists {
                return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
            }

            if !user_checks.impersonator_admin {
                return Err(BasicError::new(StatusCode::FORBIDDEN).into());
            }
        }

        TokenSource::OAuth(OAuthProvider::Google { google_id }) => {
            let checks = sqlx::query!(
                r#"
select exists(select 1 from user_auth_google where user_id = $1 and google_id = $2) as "user_valid_oauth!"
"#,
                claims.sub,
                google_id
            )
            .fetch_one(db)
            .await
            .map_err(Into::into)
            .map_err(error::ise)?;

            if !checks.user_valid_oauth {
                return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
            }
        }
    }

    Ok(claims)
}

pub async fn check_signup_token(
    db: &PgPool,
    token_string: &str,
    csrf: &str,
    token_key: &[u8; 32],
) -> Result<OAuthSignupClaims, actix_web::Error> {
    let token = validate_token(token_string, OAUTH_SIGNUP_FOOTER, token_key)?;

    let claims: OAuthSignupClaims = serde_json::from_value(token)
        .map_err(Into::into)
        .map_err(error::ise)?;

    if claims.csrf != csrf {
        return Err(
            BasicError::with_message(StatusCode::UNAUTHORIZED, "csrf mismatch".to_owned()).into(),
        );
    }

    // fixme: handle the email belonging to a user that exists (how?)
    match &claims.provider {
        OAuthProvider::Google { google_id } => {
            let checks = sqlx::query!(
                r#"
select exists(select 1 from user_auth_google where google_id = $1) as "oauth_exists!"
"#,
                google_id
            )
            .fetch_one(db)
            .await
            .map_err(Into::into)
            .map_err(error::ise)?;

            if checks.oauth_exists {
                return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
            }
        }
    }

    Ok(claims)
}

pub fn create_signin_token(
    user_id: Uuid,
    token_secret: &[u8; 32],
    local_insecure: bool,
    source: TokenSource,
) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf = generate_csrf();

    let mut builder = PasetoBuilder::new();
    let token = base_token(&mut builder, csrf.clone(), token_secret, Duration::hours(1))
        .set_subject(&user_id.to_hyphenated().to_string())
        .set_claim("source", serde_json::to_value(source)?)
        .set_footer(AUTHORIZED_FOOTER)
        .build()
        .map_err(|err| anyhow::anyhow!("failed to create token: {}", err))?;

    Ok((
        csrf,
        create_cookie(token, local_insecure, MAX_SIGNIN_COOKIE_DURATION),
    ))
}

pub fn create_oauth_signup_token(
    email: &str,
    token_secret: &[u8; 32],
    local_insecure: bool,
    provider: OAuthProvider,
) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf = generate_csrf();

    let mut builder = PasetoBuilder::new();
    let token = base_token(&mut builder, csrf.clone(), token_secret, Duration::hours(1))
        .set_subject(email)
        .set_claim("provider", serde_json::to_value(provider)?)
        .set_footer(OAUTH_SIGNUP_FOOTER)
        .build()
        .map_err(|err| anyhow::anyhow!("failed to create token: {}", err))?;

    Ok((
        csrf,
        create_cookie(token, local_insecure, MAX_SIGNUP_COOKIE_DURATION),
    ))
}

fn base_token<'a>(
    builder: &'a mut PasetoBuilder<'a>,
    csrf: String,
    token_secret: &'a [u8; 32],
    ttl: Duration,
) -> &'a mut PasetoBuilder<'a> {
    let now = Utc::now();
    builder
        .set_issued_at(None)
        .set_expiration(&(now + ttl))
        .set_not_before(&now)
        .set_encryption_key(token_secret)
        .set_claim("csrf", serde_json::Value::String(csrf))
        .set_footer(OAUTH_SIGNUP_FOOTER)
}

fn create_cookie(token: String, local_insecure: bool, ttl: time::Duration) -> Cookie<'static> {
    let mut cookie = CookieBuilder::new(AUTH_COOKIE_NAME, token)
        .http_only(true)
        .secure(!local_insecure)
        .same_site(SameSite::Lax)
        .max_age(ttl);

    if !local_insecure {
        cookie = cookie.domain(COOKIE_DOMAIN);
    }

    cookie.finish()
}

pub fn generate_csrf() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(16).collect()
}
