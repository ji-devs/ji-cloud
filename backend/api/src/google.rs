use core::settings::GoogleOAuth;

use config::RemoteTarget;
use shared::domain::session::GetOAuthUrlKind;

use crate::error;

#[derive(serde::Deserialize, Debug)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    /// When the access token expires
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

pub fn oauth_url(remote_target: RemoteTarget, url_kind: GetOAuthUrlKind) -> String {
    let route = match url_kind {
        GetOAuthUrlKind::Register => "user/register-oauth",
        GetOAuthUrlKind::Login => "user/login-oauth",
    };

    format!("{}/{}", remote_target.frontend_url(), route)
}

pub async fn convert_oauth_code(
    config: &GoogleOAuth,
    code: &str,
) -> Result<GoogleTokenResponse, error::Service> {
    // todo: `external` route
    let resp = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("grant_type", "authorization_code"),
            ("client_id", &config.client),
            ("client_secret", &config.secret),
            (
                "redirect_uri",
                &oauth_url(RemoteTarget::Local, GetOAuthUrlKind::Login),
            ),
        ])
        .send()
        .await?;

    Ok(resp.json::<GoogleTokenResponse>().await?)
}
