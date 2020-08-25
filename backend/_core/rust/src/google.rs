use crate::env::req_env;
use anyhow::Context;
use futures_util::future::TryFutureExt;
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufReader,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize)]
pub struct GoogleCredentials {
    pub private_key: String,
    pub project_id: String,
    pub client_email: String,
}

#[derive(Deserialize)]
pub struct GoogleAccessTokenResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct GoogleSecretResponse {
    pub payload: GoogleSecretResponsePayload,
}
#[derive(Deserialize)]
pub struct GoogleSecretResponsePayload {
    pub data: String,
}

#[derive(Serialize, Debug)]
pub struct GoogleApiClaims<'a> {
    iss: &'a str,
    scope: &'static str,
    aud: &'static str,
    exp: u64,
    iat: u64,
}

impl<'a> GoogleApiClaims<'a> {
    pub fn new(credentials: &'a GoogleCredentials) -> Self {
        Self {
            iss: &credentials.client_email,
            scope: "https://www.googleapis.com/auth/cloud-platform",
            aud: "https://oauth2.googleapis.com/token",
            exp: (SystemTime::now() + Duration::from_secs(3600))
                .duration_since(UNIX_EPOCH)
                .expect("get duration")
                .as_secs(),
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("get duration")
                .as_secs(),
        }
    }
}

pub fn get_google_credentials(env_var_name: &str) -> anyhow::Result<GoogleCredentials> {
    let credentials = req_env(env_var_name)?;

    let credentials = File::open(credentials.clone())
        .with_context(|| anyhow::anyhow!("Couldn't open {}", credentials))?;

    let credentials = serde_json::from_reader(BufReader::new(credentials))?;

    Ok(credentials)
}

pub async fn get_google_token_from_credentials(
    credentials: &GoogleCredentials,
) -> anyhow::Result<String> {
    let claims = GoogleApiClaims::new(&credentials);
    let token_assertion = jwt::encode(
        &jwt::Header::new(jwt::Algorithm::RS256),
        &claims,
        &jwt::EncodingKey::from_rsa_pem(credentials.private_key.as_bytes())
            .with_context(|| anyhow::anyhow!("couldn't get encoding key".to_string()))?,
    )
    .with_context(|| anyhow::anyhow!("couldn't encode jwt for google api request"))?;

    let form = reqwest::multipart::Form::new()
        .text("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer")
        .text("assertion", token_assertion);

    let token_response: GoogleAccessTokenResponse = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .multipart(form)
        .send()
        .and_then(|res| res.json())
        .await
        .with_context(|| anyhow::anyhow!("couldn't get google access token"))?;

    Ok(token_response.access_token)
}

pub async fn get_google_token_from_metaserver() -> anyhow::Result<String> {
    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    let token_response: GoogleAccessTokenResponse = reqwest::Client::new()
        .get(url)
        .header("Metadata-Flavor", "Google")
        .send()
        .and_then(|res| res.json())
        /*
        .and_then(|res| async move {
            //res.json()
            let text = res.text().await.expect("couldn't get response text to log");
            eprintln!("raw: {}", text);
            let json = serde_json::from_str(&text).unwrap();
            Ok(json)
        })
        */
        .await
        .map_err(|err| {
            anyhow::anyhow!(
                "couldn't get google access token from metaserver: {:?}",
                err
            )
        })?;

    Ok(token_response.access_token)
}

pub fn get_project_id(credentials_env_var: Option<&str>) -> anyhow::Result<String> {
    match credentials_env_var.and_then(|var| get_google_credentials(var).ok()) {
        Some(credentials) => Ok(credentials.project_id),
        None => req_env("PROJECT_ID"),
    }
}

pub async fn get_access_token_and_project_id(
    credentials_env_var_name: &str,
) -> anyhow::Result<(String, String)> {
    let credentials = get_google_credentials(credentials_env_var_name);
    match credentials {
        Ok(credentials) => {
            let token = get_google_token_from_credentials(&credentials).await?;
            Ok((token, credentials.project_id))
        }
        Err(_) => {
            let project_id = req_env("PROJECT_ID").with_context(|| {
                anyhow::anyhow!(
                    "You must set PROJECT_ID as an env var since there's no {}",
                    credentials_env_var_name
                )
            })?;
            let token = get_google_token_from_metaserver().await?;
            Ok((token, project_id))
        }
    }
}

pub async fn get_secret(
    token: &str,
    project_id: &str,
    secret_name: &str,
) -> anyhow::Result<String> {
    // todo: skip the unwraps

    let api_name = format!(
        "projects/{}/secrets/{}/versions/latest:access",
        project_id, secret_name
    );

    let path = format!("https://secretmanager.googleapis.com/v1beta1/{}", api_name);

    let request = reqwest::Client::new()
        .get(&path)
        .header("Authorization", &format!("Bearer {}", token));

    let response: GoogleSecretResponse = request
        .send()
        .and_then(|res| res.json())
        .await
        .expect(&format!("couldn't get secret: {}", secret_name));

    let bytes: Vec<u8> = base64::decode(response.payload.data).unwrap();
    Ok(std::str::from_utf8(&bytes).unwrap().to_string())
}

#[cfg(all(test, feature = "has_google_auth"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_secrets() {
        dotenv::dotenv().ok();

        let credentials = get_google_credentials("GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX")
            .await
            .unwrap();
        let token = get_google_token_from_credentials(&credentials)
            .await
            .unwrap();
        let secret = get_secret(&token, &credentials.project_id, "SANITY_TEST").await;

        assert_eq!(secret, "hello_world");
    }
}
