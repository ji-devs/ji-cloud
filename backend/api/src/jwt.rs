use crate::{db::user::exists_by_firebase, extractor::FirebaseId};
use core::settings::SETTINGS;
use jsonwebtoken as jwt;
use shared::auth::AuthClaims;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub enum Error {
    Jwt(jwt::errors::Error),
    Csrf,
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
pub async fn check_no_csrf(db: &PgPool, token_string: &str) -> anyhow::Result<Option<AuthClaims>> {
    let claims = get_claims(token_string)
        .map_err(|e| anyhow::anyhow!("{:?}", e))
        .unwrap();

    // todo: remove the clone here... Just requires making `FirebaseId` use a Cow<'a, str> and making it impl clone, I think.
    match exists_by_firebase(db, &FirebaseId(claims.id.clone())).await? {
        true => Ok(Some(claims)),
        false => Ok(None),
    }
}
