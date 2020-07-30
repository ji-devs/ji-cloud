use crate::db::user::by_id;
use core::settings::SETTINGS;
use jsonwebtoken as jwt;
use shared::auth::AuthClaims;
use sqlx::postgres::PgPool;

pub enum Error {
    Jwt(jwt::errors::Error),
    Csrf,
    NoUser,
}

pub fn get_claims(token_string: &str) -> Result<AuthClaims, Error> {
    //see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    let key =
        jsonwebtoken::DecodingKey::from_secret(SETTINGS.get().unwrap().jwt_decoding_key.as_ref());

    let validation = jwt::Validation {
        validate_exp: false,
        ..Default::default()
    };

    jsonwebtoken::decode::<AuthClaims>(&token_string, &key, &validation)
        .map(|decoded| decoded.claims)
        .map_err(|err| Error::Jwt(err))
}

pub fn check_no_db(token_string: &str, csrf: &str) -> Result<AuthClaims, Error> {
    get_claims(token_string).and_then(|claims| {
        if claims.csrf.as_deref() == Some(csrf) {
            Ok(claims)
        } else {
            Err(Error::Csrf)
        }
    })
}
pub async fn check_no_csrf(db: &PgPool, token_string: &str) -> Result<AuthClaims, Error> {
    let claims = get_claims(token_string)?;

    // todo: handle db errors properly (by returning a error that will cause the server to 500)
    match by_id(db, &claims.id).await {
        Ok(Some(_)) => Ok(claims),
        _ => Err(Error::NoUser),
    }
}
