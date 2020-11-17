use crate::env::req_env;
use anyhow::{anyhow, Context};
use futures_util::future::TryFutureExt;
use reqwest::StatusCode;
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

/// Attempts to load an access token and project id from the given env var.
pub async fn get_access_token_and_project_id(
    credentials_env_key: &str,
) -> anyhow::Result<(String, String)> {
    match req_env(credentials_env_key) {
        Ok(credentials_file) => {
            let credentials = yup_oauth2::read_service_account_key(credentials_file).await?;

            let project_id = credentials
                .project_id
                .clone()
                .ok_or_else(|| anyhow!("Couldn't find project_id"))?;

            let token = get_google_token_from_credentials(credentials).await?;

            Ok((token.as_str().to_owned(), project_id))
        }

        Err(_) => {
            let token = get_google_token_from_metaserver().await?;
            let project_id = req_env("PROJECT_ID")?;

            Ok((token, project_id))
        }
    }
}

pub(crate) async fn get_google_token_from_metaserver() -> anyhow::Result<String> {
    #[derive(Deserialize)]
    struct GoogleAccessTokenResponse {
        access_token: String,
    }

    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    let token_response: GoogleAccessTokenResponse = reqwest::Client::new()
        .get(url)
        .header("Metadata-Flavor", "Google")
        .send()
        .and_then(|res| res.json())
        .await
        .map_err(|err| {
            anyhow::anyhow!(
                "couldn't get google access token from metaserver: {:?}",
                err
            )
        })?;

    Ok(token_response.access_token)
}

/// Gets `secret_name` from GCS via the given `token` and `project_id`, returning `None` if the secret doesn't exist.
/// # Errors:
/// If the request fails
/// If the request's response is invalid
/// If the bytes of the response data are not valid UTF-8
pub(crate) async fn get_optional_secret(
    token: &str,
    project_id: &str,
    secret_name: &str,
) -> anyhow::Result<Option<String>> {
    let path = format!(
        "https://secretmanager.googleapis.com/v1/projects/{}/secrets/{}/versions/latest:access",
        project_id, secret_name
    );

    let request = reqwest::Client::new()
        .get(&path)
        .header("Authorization", &format!("Bearer {}", token));

    let response = request
        .send()
        .await
        .with_context(|| anyhow!("request to get secret failed"))?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let response: GoogleSecretResponse = response
        .json()
        .await
        .with_context(|| anyhow!("failed to decode response for secret"))?;

    let bytes: Vec<u8> = base64::decode(response.payload.data)?;

    Ok(Some(String::from_utf8(bytes)?))
}

/// Gets `secret_name` from GCS via the given `token` and `project_id`
/// # Errors
/// If the request fails
/// If the request's response is invalid
/// If the bytes of the response data are not valid UTF-8
/// If the secret doesn't exist.
pub async fn get_secret(
    token: &str,
    project_id: &str,
    secret_name: &str,
) -> anyhow::Result<String> {
    get_optional_secret(token, project_id, secret_name)
        .await?
        .ok_or_else(|| anyhow!("secret `{}` not present", secret_name))
}
