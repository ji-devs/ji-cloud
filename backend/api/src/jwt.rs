use crate::db;
use jsonwebtoken::{self as jwt, DecodingKey};
use shared::domain::auth::AuthClaims;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub enum Error {
    Jwt(jwt::errors::Error),
    // Csrf,
}

pub fn get_claims(token_string: &str, key: DecodingKey) -> Result<AuthClaims, Error> {
    //see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881

    let validation = jwt::Validation {
        validate_exp: false,
        ..Default::default()
    };

    jsonwebtoken::decode::<AuthClaims>(&token_string, &key, &validation)
        .map(|decoded| decoded.claims)
        .map_err(|err| Error::Jwt(err))
}

pub fn check_no_db(
    token_string: &str,
    csrf: &str,
    key: DecodingKey,
) -> Result<Option<AuthClaims>, Error> {
    let claims = get_claims(token_string, key)?;
    if claims.csrf.as_deref() == Some(csrf) {
        Ok(Some(claims))
    } else {
        Ok(None)
    }
}
pub async fn check_no_csrf<'a>(
    db: &PgPool,
    token_string: &str,
    key: DecodingKey<'a>,
) -> anyhow::Result<Option<AuthClaims>> {
    let claims = get_claims(token_string, key)
        .map_err(|e| anyhow::anyhow!("{:?}", e))
        .unwrap();

    match db::user::exists(db, claims.id).await? {
        true => Ok(Some(claims)),
        false => Ok(None),
    }
}
