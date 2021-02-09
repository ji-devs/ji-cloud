use http::StatusCode;
use paseto::TimeBackend;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::BasicError;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The claims that are used as part of the user's token.
pub struct TokenClaims {
    /// The user claimed by the token.
    pub sub: Uuid,

    /// The csrf that must match for the token to be considered valid.
    pub csrf: String,

    /// What initially created this token (todo: replace with a session key or smth)
    pub source: TokenSource,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum TokenSource {
    Basic,
    Impersonate(Uuid),
}

// todo: accept a transaction instead so that we can do `for share` row locks
pub async fn check_token(
    db: &PgPool,
    token_string: &str,
    csrf: &str,
    token_key: &[u8; 32],
) -> Result<TokenClaims, actix_web::Error> {
    let token = paseto::validate_local_token(token_string, None, token_key, &TimeBackend::Chrono)
        .map_err(|_| BasicError::new(StatusCode::UNAUTHORIZED))?;

    let claims: TokenClaims = serde_json::from_value(token)
        .map_err(Into::into)
        .map_err(crate::error::ise)?;

    if claims.csrf != csrf {
        return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
    }

    // todo: have some kind of session storage, etc, change this to a session instead/in addition to a source.
    match claims.source {
        TokenSource::Basic => {
            let exists = sqlx::query!(
                r#"select exists(select 1 from user_auth_basic where user_id = $1) as "exists!""#,
                claims.sub,
            )
            .fetch_one(db)
            .await
            .map_err(Into::into)
            .map_err(crate::error::ise)?;

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
            .map_err(crate::error::ise)?;

            if !user_checks.impersonated_exists {
                return Err(BasicError::new(StatusCode::UNAUTHORIZED).into());
            }

            if !user_checks.impersonator_admin {
                return Err(BasicError::new(StatusCode::FORBIDDEN).into());
            }
        }
    }

    Ok(claims)
}
