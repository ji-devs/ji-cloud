use core::settings::SETTINGS;
use futures_util::future::TryFutureExt;
use jsonwebtoken as jwt;
use serde::Deserialize;

#[derive(Deserialize)]
struct JsApiResponse {
    valid: bool,
}

#[derive(Debug, Deserialize)]
struct FirebaseClaims {
    sub: String,
}

pub async fn get_firebase_id(token: &str) -> anyhow::Result<Option<String>> {
    //use the js server to handle this, since it has the official firebase admin sdk
    //it could be done natively in Rust, but depends on:
    //1. https://github.com/Keats/jsonwebtoken/issues/127
    //2. all the specific claim checks (e.g. timestamp comparisons)

    let response: JsApiResponse = reqwest::Client::new()
        .get(&format!(
            "{}/validate-firebase-token/{}",
            SETTINGS.get().unwrap().remote_target.api_js_url(),
            token
        ))
        .header(
            "INTER_SERVER_SECRET",
            &SETTINGS.get().unwrap().inter_server_secret,
        )
        .send()
        .and_then(|res| res.json::<JsApiResponse>())
        .await
        .map_err(|err| {
            log::warn!("js/firebase error, shouldn't happen: {:?}", err);
            err
        })?;

    if response.valid {
        let claims: FirebaseClaims = jwt::dangerous_insecure_decode(&token)?.claims;

        let user_id = claims.sub;

        Ok(Some(user_id))
    } else {
        Ok(None)
    }
}
