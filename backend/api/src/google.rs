use std::collections::HashMap;

use anyhow::Context;
use config::RemoteTarget;
use core::settings::GoogleOAuth;
use http::StatusCode;
use shared::domain::session::OAuthUrlKind;

use crate::error;

#[derive(serde::Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    /// When the access token expires
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum TokenErrorResponse {
    Known(TokenErrorKind),
    Unknown(HashMap<String, String>),
}

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "error")]
#[serde(rename_all = "snake_case")]
pub enum TokenErrorKind {
    // only known error description is "Bad Request"
    InvalidGrant,

    // only known error description is "Bad Request"
    RedirectUriMismatch,

    InvalidClient { error_description: String },
}

pub fn oauth_url(remote_target: RemoteTarget, url_kind: OAuthUrlKind) -> String {
    let route = match url_kind {
        OAuthUrlKind::Register => "user/register-oauth",
        OAuthUrlKind::Login => "user/login-oauth",
    };

    format!("{}/{}", remote_target.pages_url(), route)
}

pub async fn convert_oauth_code(
    config: &GoogleOAuth,
    code: &str,
    redirect_url: &str,
) -> Result<TokenResponse, error::GoogleOAuth> {
    // todo: `external` route
    let resp: reqwest::Response = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("grant_type", "authorization_code"),
            ("client_id", &config.client),
            ("client_secret", &config.secret),
            ("redirect_uri", &redirect_url),
        ])
        .send()
        .await?;

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<TokenResponse>().await?),
        _ => {
            let err = resp.json::<TokenErrorResponse>().await.with_context(|| {
                anyhow::anyhow!("Failed to decode {}", stringify!(TokenErrorResponse))
            })?;

            Err(err.into())
        }
    }
}
