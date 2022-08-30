use crate::env::req_env;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Duration, Utc};
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

/// Attemps to fetch the oauth2 token given a credentials file.
pub async fn get_google_token_from_credentials(
    credentials: ServiceAccountKey,
) -> anyhow::Result<AccessToken> {
    let token = ServiceAccountAuthenticator::builder(credentials)
        .build()
        .await?
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await?;

    Ok(token)
}

/// Represents the response for fetching an access token from the google metadata server in a remote
/// container.
pub struct GoogleAccessTokenResponse {
    /// Token to call GCP services with.
    /// TODO: make this an Enum over Metadata, Service Account json key
    pub access_token: Option<String>,
    /// Time that the token expires at. If `None` this does not mean that the token does not expire.
    pub expires_at: Option<DateTime<Utc>>,
}

/// Attempts to load an access token and project id from the given env var.
///
/// This will include an `expires_at` (Utc) field when fetched from the google metadata
/// server when running inside Cloud Run `RemoteTarget::Sandbox | RemoteTarget::Relase`.
pub async fn get_access_token_response_and_project_id(
    credentials_env_key: &str,
) -> anyhow::Result<(GoogleAccessTokenResponse, String)> {
    match req_env(credentials_env_key) {
        Ok(credentials_file) => {
            // TODO: handle expiry, token refresh when reading from service account
            let credentials = yup_oauth2::read_service_account_key(credentials_file).await?;

            let project_id = credentials
                .project_id
                .clone()
                .ok_or_else(|| anyhow!("Couldn't find project_id"))?;

            let token = get_google_token_from_credentials(credentials).await?;

            let access_token = Some(token.as_str().to_owned());
            let expires_at = if let Some(exp_at) = token.expiration_time() {
                Some(Utc::now() + Duration::seconds(exp_at.unix_timestamp()))
            } else {
                None
            };

            Ok((
                GoogleAccessTokenResponse {
                    access_token,
                    expires_at,
                },
                project_id,
            ))
        }

        Err(_) => {
            let token_response = get_google_token_response_from_metadata_server()
                .await
                .with_context(|| {
                    anyhow::anyhow!("couldn't get google access token from metaserver",)
                })?;

            let project_id = req_env("PROJECT_ID")?;

            Ok((token_response, project_id))
        }
    }
}

/// Attempts to load an access token from the google metadata server. This is only available when
/// when running inside Cloud Run `RemoteTarget::Sandbox | RemoteTarget::Release`.
pub async fn get_google_token_response_from_metadata_server(
) -> anyhow::Result<GoogleAccessTokenResponse> {
    #[derive(Deserialize)]
    struct InnerGoogleAccessTokenResponse {
        access_token: String,
        expires_in: i64,
    }

    let url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";

    let token_response: InnerGoogleAccessTokenResponse = reqwest::Client::new()
        .get(url)
        .header("Metadata-Flavor", "Google")
        .send()
        .await?
        .json()
        .await?;

    let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

    Ok(GoogleAccessTokenResponse {
        access_token: Some(token_response.access_token),
        expires_at: Some(expires_at),
    })
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

    println!("get optional secret status: {:?}", response.status());

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
