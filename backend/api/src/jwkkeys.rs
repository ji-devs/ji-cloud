use crate::extractor::FirebaseId;
use anyhow::{anyhow, bail};
use jsonwebtoken as jwt;
use jwt::{Algorithm, DecodingKey, TokenData, Validation};
use reqwest::{header, Response};
use serde::Deserialize;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::RwLock, task::JoinHandle};

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize)]
struct JwkKey {
    e: String,
    alg: Algorithm,
    kid: String,
    n: String,
}

#[derive(Debug)]
struct JwkKeys {
    keys: Vec<JwkKey>,
    expiration_time: Instant,
}

impl JwkKeys {
    const RETRY_AFTER: Duration = Duration::from_millis(50);

    /// Errors if the keys have expired, in which case,
    /// wait until the returned instant has passed before retrying
    fn get_key(&self, kid: &str) -> Result<Option<&JwkKey>, Instant> {
        let now = Instant::now();
        if self.expiration_time > now {
            Ok(self.keys.iter().find(|it| it.kid == kid))
        } else {
            Err(now + Self::RETRY_AFTER)
        }
    }
}

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

async fn fetch_keys_for_config() -> anyhow::Result<JwkKeys> {
    let http_response = reqwest::get(config::JWK_URL).await?;
    let now = Instant::now();
    let max_age = get_max_age(&http_response).unwrap_or(FALLBACK_TIMEOUT);
    let resp: KeyResponse = http_response.json().await?;

    Ok(JwkKeys {
        keys: resp.keys,
        expiration_time: now + max_age,
    })
}

// Determines the max age of an HTTP response
fn get_max_age(response: &Response) -> anyhow::Result<Duration> {
    response
        .headers()
        .get(header::CACHE_CONTROL)
        .ok_or_else(|| anyhow!("No Cache Control Header"))?
        .to_str()
        .map(parse_max_age_value)?
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

#[derive(Debug)]
pub struct JwkVerifier {
    // use a vec instead of a hashmap because there are typically
    // _very_ few keys, so looking them up in a hashmap is likely slower.
    key_holder: RwLock<JwkKeys>,
    issuer: String,
    audience: String,
}

impl JwkVerifier {
    fn new(issuer: String, audience: String) -> JwkVerifier {
        JwkVerifier {
            key_holder: RwLock::new(JwkKeys {
                keys: vec![],
                expiration_time: Instant::now(),
            }),
            issuer,
            audience,
        }
    }

    pub async fn verify(&self, token: &str, max_attempts: usize) -> anyhow::Result<FirebaseId> {
        let token_kid = jwt::decode_header(token)
            .map_err(|e| anyhow!("error decoding jwt header: {}", e))?
            .kid
            .ok_or_else(|| anyhow!("Missing Key ID"))?;

        for _ in 0..(max_attempts.max(1)) {
            match self.key_holder.read().await.get_key(&token_kid) {
                Ok(Some(key)) => {
                    let claims: Claims = self.decode_token_with_key(key, token)?.claims;
                    let now = chrono::Utc::now().timestamp() as u64;

                    if claims.auth_time > now || claims.iat > now {
                        bail!("token isn't valid yet")
                    }

                    return Ok(FirebaseId(claims.sub));
                }
                Ok(None) => bail!("invalid KID"),
                Err(delay_until) => tokio::time::delay_until(delay_until.into()).await,
            }
        }

        bail!("failed to get keys")
    }

    async fn set_keys(&self, keys: JwkKeys) {
        *self.key_holder.write().await = keys;
    }

    fn decode_token_with_key(
        &self,
        key: &JwkKey,
        token: &str,
    ) -> anyhow::Result<TokenData<Claims>> {
        let mut validation = Validation::new(key.alg);
        validation.set_audience(&[&self.audience]);
        validation.iss = Some(self.issuer.clone());

        let key = DecodingKey::from_rsa_components(&key.n, &key.e);
        return jwt::decode(token, &key, &validation).map_err(|e| anyhow!(e));
    }
}

pub fn run_task(verifier: Arc<JwkVerifier>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            log::trace!("Getting keys for jwk");

            match fetch_keys_for_config().await {
                Ok(keys) => {
                    let refresh_at = keys.expiration_time;

                    verifier.set_keys(keys).await;

                    tokio::time::delay_until(refresh_at.into()).await;
                }

                Err(e) => {
                    log::error!("Error in jwk key-fetch task: {}", e);
                    tokio::time::delay_for(Duration::from_secs(5)).await;
                }
            };
        }
    })
}

pub fn create_verifier(config: core::settings::JwkSettings) -> Arc<JwkVerifier> {
    Arc::new(JwkVerifier::new(config.issuer, config.audience))
}
