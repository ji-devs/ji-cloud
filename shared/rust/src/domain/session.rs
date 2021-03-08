//! types for Auth / session management

use std::fmt;

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// The name to use for auth cookies.
pub const AUTH_COOKIE_NAME: &str = "X-AUTH";

/// The name of the CSRF header.
pub const CSRF_HEADER_NAME: &str = "X-CSRF";

#[deprecated(note = "use NewSessionResponse")]
pub use NewSessionResponse as CreateSessionSuccess;

/// Response for creating a session
///
/// Note: This response *also* includes a cookie.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum CreateSessionResponse {
    /// A new session was successfully created and the user may use the api as normal.
    Login(NewSessionResponse),

    /// The user has no profile, a token for creating one has been returned
    Register(NewSessionResponse),
}

/// Response for successfully creating a session.
/// Note: This response *also* includes a cookie.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct NewSessionResponse {
    /// A transparent CSRF token to use for this Session.
    pub csrf: String,
}

/// Which URL to use for OAuth callback.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum OAuthUrlKind {
    /// Get OAuth Url for login
    Login,
    /// Get OAuth Url for register
    Register,
}

/// Which *service* to use for OAuth Url generation.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[non_exhaustive]
pub enum GetOAuthUrlServiceKind {
    /// Google OAuth v2
    Google,
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
        /// The google OAuth Code
        code: String,

        /// Which OAuth url was used
        redirect_kind: OAuthUrlKind,
    },
}

impl fmt::Debug for CreateSessionOAuthRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // todo: replace with `finish_non_exhaustive`
            Self::Google { .. } => f.debug_struct("Google").finish(),
        }
    }
}

/// Response for successfully creating a session / signing in, via oauth.
/// Note: This response *also* includes a cookie.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[deprecated(note = "use CreateSessionResponse")]
#[non_exhaustive]
pub enum CreateSessionOAuthResponse {
    /// Successfully logged in.
    #[serde(rename = "login")]
    Login {
        /// A transparent CSRF token to use for this Session.
        csrf: String,
    },

    /// Failed to log in; a token for creating a user has been returned.
    #[serde(rename = "register")]
    CreateUser {
        /// A transparent CSRF token to use for this Session.
        csrf: String,
    },
}
