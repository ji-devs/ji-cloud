use crate::db;
use jsonwebtoken::{self as jwt, DecodingKey};
use shared::domain::auth::AuthClaims;
use sqlx::postgres::PgPool;

pub fn get_claims(
    token_string: &str,
    key: &DecodingKey<'_>,
) -> Result<AuthClaims, jwt::errors::Error> {
    let validation = jwt::Validation {
        validate_exp: false,
        ..Default::default()
    };

    jsonwebtoken::decode::<AuthClaims>(token_string, key, &validation).map(|decoded| decoded.claims)
}

pub fn check_no_db(
    token_string: &str,
    csrf: &str,
    key: &DecodingKey<'_>,
) -> Result<Option<AuthClaims>, jwt::errors::Error> {
    let claims = get_claims(token_string, key)?;
    if claims.csrf.as_deref() == Some(csrf) {
        Ok(Some(claims))
    } else {
        Ok(None)
    }
}
pub async fn check_no_csrf(
    db: &PgPool,
    token_string: &str,
    key: &DecodingKey<'_>,
) -> anyhow::Result<Option<AuthClaims>> {
    let claims = get_claims(token_string, key)?;

    match db::user::exists(db, claims.id).await? {
        true => Ok(Some(claims)),
        false => Ok(None),
    }
}
