use crate::extractor::FirebaseId;
use anyhow::{anyhow, bail};
use jsonwebtoken as jwt;
use jwt::{Algorithm, DecodingKey, TokenData, Validation};
use reqwest::Response;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::RwLock, task::JoinHandle};

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

pub struct JwkKeys {
    pub keys: Vec<JwkKey>,
    pub expiration_time: Instant,
}

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

pub async fn fetch_keys_for_config() -> anyhow::Result<JwkKeys> {
    let http_response = reqwest::get(config::JWK_URL).await?;
    let now = Instant::now();
    let max_age = get_max_age(&http_response).unwrap_or(FALLBACK_TIMEOUT);
    let expire_at = now - max_age;
    let result = Result::Ok(http_response.json::<KeyResponse>().await?);

    result.map(|res| JwkKeys {
        keys: res.keys,
        expiration_time: expire_at,
    })
}

// Determines the max age of an HTTP response
fn get_max_age(response: &Response) -> anyhow::Result<Duration> {
    let headers = response.headers();
    let header = headers.get("Cache-Control");
    let header = header.ok_or_else(|| anyhow!("No Cache Control Header"))?;

    parse_max_age_value(header.to_str()?)
}

fn parse_max_age_value(cache_control_value: &str) -> anyhow::Result<Duration> {
    for token in cache_control_value.split(",") {
        let mut key_value = token.split("=").map(str::trim);
        let key = key_value
            .next()
            .expect("str split always gives at least one element");

        if key.eq_ignore_ascii_case("max-age") {
            let value = key_value
                .next()
                .ok_or_else(|| anyhow!("max-age is missing"))?;
            return Ok(Duration::from_secs(value.parse()?));
        }
    }

    bail!("No max-age specified");
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub auth_time: u64,
}

fn get_current_timestamp() -> u64 {
    let start = std::time::SystemTime::now();
    start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[derive(Debug)]
pub struct JwkVerifier {
    keys: HashMap<String, JwkKey>,
    issuer: String,
    audience: String,
}

fn keys_to_map(keys: Vec<JwkKey>) -> HashMap<String, JwkKey> {
    let mut keys_as_map = HashMap::new();
    for key in keys {
        keys_as_map.insert(key.kid.clone(), key);
    }

    keys_as_map
}

impl JwkVerifier {
    pub fn new(keys: Vec<JwkKey>, issuer: String, audience: String) -> JwkVerifier {
        JwkVerifier {
            keys: keys_to_map(keys),
            issuer,
            audience,
        }
    }

    pub fn verify(&self, token: &str) -> anyhow::Result<FirebaseId> {
        let token_kid = jwt::decode_header(token)
            .map_err(|e| anyhow!("error decoding jwt header: {}", e))?
            .kid
            .ok_or_else(|| anyhow!("Missing Key ID"))?;

        let jwk_key = self
            .keys
            .get(&token_kid)
            .ok_or_else(|| anyhow!("Invalid Key ID"))?;

        let claims: Claims = self.decode_token_with_key(jwk_key, token)?.claims;
        let now = get_current_timestamp();
        if claims.auth_time > now || claims.iat > now {
            bail!("token isn't valid yet")
        }

        Ok(FirebaseId(claims.sub))
    }

    pub fn set_keys(&mut self, keys: Vec<JwkKey>) {
        self.keys = keys_to_map(keys);
    }

    fn decode_token_with_key(
        &self,
        key: &JwkKey,
        token: &str,
    ) -> anyhow::Result<TokenData<Claims>> {
        let algorithm = Algorithm::from_str(&key.alg).map_err(|e| anyhow!("{}", e))?;

        let mut validation = Validation::new(algorithm);
        validation.set_audience(&[&self.audience]);
        validation.iss = Some(self.issuer.clone());
        let key = DecodingKey::from_rsa_components(&key.n, &key.e);
        return jwt::decode::<Claims>(token, &key, &validation).map_err(|e| anyhow!("{}", e));
    }
}

pub fn run_task(verifier: Arc<RwLock<JwkVerifier>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let keys = match fetch_keys_for_config().await {
                Ok(keys) => keys,
                Err(e) => {
                    log::error!("Error in jwk key-fetch task: {}", e);
                    tokio::time::delay_for(Duration::from_secs(5).into()).await;
                    continue;
                }
            };

            verifier.write().await.set_keys(keys.keys);

            tokio::time::delay_until(keys.expiration_time.into()).await;
        }
    })
}

pub async fn create_verifier(
    config: core::settings::JwkSettings,
) -> anyhow::Result<Arc<RwLock<JwkVerifier>>> {
    let keys: JwkKeys = fetch_keys_for_config().await?;

    let verifier = JwkVerifier::new(keys.keys, config.issuer, config.audience);
    Ok(Arc::new(RwLock::new(verifier)))
}
