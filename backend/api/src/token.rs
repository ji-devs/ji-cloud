use actix_http::cookie::{Cookie, CookieBuilder, SameSite};
use chrono::{Duration, Utc};
use http::StatusCode;
use paseto::{PasetoBuilder, TimeBackend};
use rand::Rng;
use shared::domain::{session::AUTH_COOKIE_NAME, user::UserScope};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{self, BasicError};

const AUTHORIZED_FOOTER: &str = "authorized";

pub struct SessionClaims {
    pub user_id: Uuid,
    pub token: String,
}

/// The claims that are used as part of the user's token.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct AuthorizedTokenClaims {
    /// The session this token is for.
    pub sub: String,

    /// The csrf that must match for the token to be considered valid.
    csrf: String,
}

bitflags::bitflags! {
    #[derive(sqlx::Type)]
    pub struct SessionMask: i16 {
        const GENERAL_API = 0b0000_0000_0000_0001;
        const PUT_PROFILE = 0b0000_0000_0000_0010;
        const VERIFY_EMAIL = 0b0000_0000_0000_0100;
        const RESET_PASSWORD = 0b0000_0000_0000_1000;
        const DELETE_ACCOUNT = 0b0000_0000_0001_0000;

        const GENERAL = Self::GENERAL_API.bits | Self::DELETE_ACCOUNT.bits;
        const ONE_TIME = Self::RESET_PASSWORD.bits | Self::VERIFY_EMAIL.bits;
    }
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
    min_mask: SessionMask,
) -> Result<SessionClaims, actix_web::Error> {
    let token = validate_token(token_string, AUTHORIZED_FOOTER, token_key)?;

    let claims: AuthorizedTokenClaims = serde_json::from_value(token)
        .map_err(Into::into)
        .map_err(error::ise)?;

    if claims.csrf != csrf {
        return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
    }

    let mut txn = db.begin().await.map_err(Into::into).map_err(error::ise)?;

    let session_info = sqlx::query!(
        r#"
select user_id
from session
where 
    token = $1 and
    expires_at < now() is not true and
    (scope_mask & $2) = $2 and
    (impersonator_id is null or exists(select 1 from user_scope where user_scope.user_id = impersonator_id and user_scope.scope = $3))
"#,
        &claims.sub,
        min_mask.bits as i16,
        UserScope::Admin as i16
    )
    .fetch_optional(&mut txn)
    .await
    .map_err(anyhow::Error::from)
    .map_err(error::ise)?
    .ok_or_else(|| BasicError::new(StatusCode::UNAUTHORIZED))?;

    if min_mask.intersects(SessionMask::ONE_TIME) {
        sqlx::query!("delete from session where token = $1", &claims.sub)
            .execute(&mut txn)
            .await
            .map_err(Into::into)
            .map_err(error::ise)?;
    }

    Ok(SessionClaims {
        user_id: session_info.user_id,
        token: claims.sub,
    })
}

pub fn create_auth_token(
    token_secret: &[u8; 32],
    local_insecure: bool,
    valid_duration: Duration,
    session: &str,
) -> anyhow::Result<(String, Cookie<'static>)> {
    let csrf = generate_csrf();

    let now = Utc::now();

    let token = PasetoBuilder::new()
        .set_expiration(&(now + valid_duration))
        .set_not_before(&now)
        .set_issued_at(Some(now))
        .set_encryption_key(token_secret)
        .set_claim("csrf", serde_json::Value::String(csrf.clone()))
        .set_subject(&session)
        .set_footer(AUTHORIZED_FOOTER)
        .build()
        .map_err(|err| anyhow::anyhow!("failed to create token: {}", err))?;

    let valid_duration = time::Duration::seconds(valid_duration.num_seconds());

    Ok((csrf, create_cookie(token, local_insecure, valid_duration)))
}

#[must_use]
fn create_cookie(token: String, local_insecure: bool, ttl: time::Duration) -> Cookie<'static> {
    CookieBuilder::new(AUTH_COOKIE_NAME, token)
        .http_only(true)
        .secure(!local_insecure)
        .same_site(SameSite::Lax)
        .max_age(ttl)
        .path("/")
        .finish()
}

#[must_use]
fn generate_csrf() -> String {
    let mut bytes = [0_u8; 32];
    rand::thread_rng().fill(&mut bytes[..]);
    base64::encode_config(&bytes, base64::URL_SAFE)
}
