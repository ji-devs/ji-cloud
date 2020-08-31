use crate::env::req_env;
use anyhow::{anyhow, Context};
use futures_util::future::TryFutureExt;
use serde::Deserialize;
use yup_oauth2::{AccessToken, ServiceAccountAuthenticator, ServiceAccountKey};

#[derive(Deserialize)]
pub(crate) struct GoogleSecretResponse {
    pub payload: GoogleSecretResponsePayload,
}
#[derive(Deserialize)]
pub(crate) struct GoogleSecretResponsePayload {
    pub data: String,
}

async fn get_google_token_from_credentials(
    credentials: ServiceAccountKey,
) -> anyhow::Result<AccessToken> {
    let token = ServiceAccountAuthenticator::builder(credentials)
        .build()
        .await?
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await?;

    Ok(token)
}

pub async fn get_access_token_and_project_id(
    credentials_env_key: &str,
) -> anyhow::Result<(AccessToken, String)> {
    let credentials = yup_oauth2::read_service_account_key(req_env(credentials_env_key)?).await?;

    let project_id = credentials
        .project_id
        .clone()
        .ok_or_else(|| anyhow!("Couldn't find project_id"))?;

    let token = get_google_token_from_credentials(credentials).await?;
    Ok((token, project_id))
}

pub async fn get_secret(
    token: &AccessToken,
    project_id: &str,
    secret_name: &str,
) -> anyhow::Result<String> {
    anyhow::ensure!(!token.is_expired(), "Token is expired");

    let path = format!("https://secretmanager.googleapis.com/v1beta1/projects/{}/secrets/{}/versions/latest:access", project_id, secret_name);

    let request = reqwest::Client::new()
        .get(&path)
        .header("Authorization", &format!("Bearer {}", token.as_str()));

    let response: GoogleSecretResponse = request
        .send()
        .and_then(|res| res.json())
        .await
        .with_context(|| anyhow!("couldn't get secret: {}", secret_name))?;

    let bytes: Vec<u8> = base64::decode(response.payload.data)?;
    Ok(String::from_utf8(bytes)?)
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
