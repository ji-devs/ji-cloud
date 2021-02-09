//! types for Auth / session management

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Response for successfully signing in.
/// Note: This response *also* includes a cookie.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CreateSessionSuccess {
    /// A transparent CSRF token to use for this Session.
    pub csrf: String,
}

/// Which URL to use for OAuth callback.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum GetOAuthUrlKind {
    /// Get OAuth Url for login
    Login,
    /// Get OAuth Url for register
    Register,
}

/// Response for what URL to use for OAuth callback.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct GetOAuthUrlResponse {
    ///  URL to use for OAuth callback
    pub url: String,
}

/// Request for Creating a Session / signing in via oauth.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[non_exhaustive]
pub enum CreateSessionOAuthRequest {
    /// OAuth with google
    Google {
        /// The google OAuth token
        token: String,
    },
}

/// Response for successfully creating a session / signing in, via oauth.
/// Note: This response *also* includes a cookie.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[non_exhaustive]
pub enum CreateSessionOAuthResponse {
    /// Successfully logged in.
    Login {
        /// A transparent CSRF token to use for this Session.
        csrf: String,
    },

    /// Failed to log in; a token for creating a user has been returned.
    CreateUser {
        /// A transparent CSRF token to use for this Session.
        csrf: String,

        /// The detected Given Name (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        given_name: Option<String>,

        /// The detected Family Name (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        family_name: Option<String>,
    },
}
