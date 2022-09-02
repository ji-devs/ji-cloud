//! types for Auth / session management

use std::fmt;

use super::user::UserId;
use crate::api::endpoints::PathPart;
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

/// The query key for
pub const AUTH_QUERY_NAME: &str = "access_token";

/// The name to use for auth cookies.
pub const AUTH_COOKIE_NAME: &str = "X-AUTH";

/// The name of the CSRF header.
pub const CSRF_HEADER_NAME: &str = "X-CSRF";

make_path_parts!(ImpersonatePath => "/v1/admin/session/user/{}" => UserId);

#[deprecated(note = "use NewSessionResponse")]
pub use NewSessionResponse as CreateSessionSuccess;

make_path_parts!(CreateSessionPath => "/v1/session");

/// Response for creating a session.
///
/// Notes:
/// * When creating a `Register` session through OAuth, the API also returns user profile info
/// given as part of the identity claims from the OAuth provider (e.g. Google).
/// * This response *also* includes a cookie.
///
/// Returned cookie auth token can be passed to the API in three ways in requests.
/// They are listed below in order of precedence (i.e. if 1 exists then 2, 3 are ignored):
///
/// 1. Passed as a query  `<uri-to-resource>?access_token=<token>`.
/// 2. Passed in the request header as `Authorization: Bearer <token>`.
/// 3. As a cookie, `X-AUTH=<token>`. This token will also be authenticated against the CSRF-prevention
/// header.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum CreateSessionResponse {
    /// A new session was successfully created and the user may use the api as normal.
    Login(NewSessionResponse),

    /// The user has no profile, a token for creating one has been returned.
    /// * If using OAuth, then a [`Some(OAuthUserProfile)`](OAuthUserProfile) is included as well
    /// containing known information about the user.
    /// * If using Basic auth, then the `oauth_profile` field will be `None` and not be serialized.
    ///
    /// ## Json response without OAuth profile:
    /// ```json
    /// {
    ///     "register": {
    ///         "csrf": <CSRF_TOKEN>,
    ///     }
    /// }
    /// ```
    ///
    /// ## Json response with OAuth profile:
    /// ```json
    /// {
    ///     "register": {
    ///         "csrf": <CSRF_TOKEN>,
    ///         "oauth_profile": {
    ///             "email": <EMAIL>,
    ///             ... # other optional profile fields
    ///         }
    ///     }
    /// }
    /// ```
    Register {
        /// Csrf token. Note that this field is "flattened" into it's contents when (de)serialized. See example above.
        #[serde(flatten)]
        response: NewSessionResponse,
        /// Oauth profile
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth_profile: Option<OAuthUserProfile>,
    },
}

/// User's profile info fetched from the OAuth service. Returned as part of the identity claims
/// to be used as defaults for populating a `PutProfile` request.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthUserProfile {
    /// The user's email
    pub email: String,

    /// The user's name
    #[serde(default)]
    pub name: Option<String>,

    /// the user's profile picture
    #[serde(default)]
    pub profile_picture: Option<String>,

    /// The user's given / first name
    #[serde(default)]
    pub given_name: Option<String>,

    /// The user's family / last name
    #[serde(default)]
    pub family_name: Option<String>,

    /// The user's locale
    #[serde(default)]
    pub locale: Option<String>,
}

/// Response for successfully creating a session.
///
/// Note: This response *also* includes a cookie.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionResponse {
    /// A transparent CSRF token to use for this Session.
    pub csrf: String,
}

/// Which URL to use for OAuth callback.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OAuthUrlKind {
    /// Get OAuth Url for login
    Login,
    /// Get OAuth Url for register
    Register,
}
impl PathPart for OAuthUrlKind {
    const PLACEHOLDER: &'static str = "{url_kind}";

    fn get_path_string(&self) -> String {
        match self {
            Self::Login => String::from("login"),
            Self::Register => String::from("register"),
        }
    }
}

/// Which *service* to use for OAuth Url generation.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum GetOAuthUrlServiceKind {
    /// Google OAuth v2
    Google,
}
impl PathPart for GetOAuthUrlServiceKind {
    const PLACEHOLDER: &'static str = "{service_kind}";

    fn get_path_string(&self) -> String {
        match self {
            Self::Google => String::from("google"),
        }
    }
}

/// OAuth provider for emails
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OAuthProvider {
    /// Google OAuth v2
    Google,
}

impl OAuthProvider {
    #[allow(missing_docs)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Google => "Google",
        }
    }
}

make_path_parts!(GetOAuthPath => "/v1/session/oauth/url/{}/{}" => GetOAuthUrlServiceKind, OAuthUrlKind);

/// Response for what URL to use for OAuth callback.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOAuthUrlResponse {
    ///  URL to use for OAuth callback
    pub url: String,
}

make_path_parts!(CreateSessionOAuthPath => "/v1/session/oauth");

/// Request for Creating a Session / signing in via oauth.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum CreateSessionOAuthRequest {
    /// OAuth with google
    Google {
        /// The google OAuth Code
        code: String,

        /// Which OAuth url was used
        /// Not sure if this is needed anymore
        redirect_kind: OAuthUrlKind,
    },
}

/// Optional query used as the first option for authentication with the API
#[derive(Serialize, Deserialize)]
pub struct SessionTokenQuery {
    /// The token to be used for authentication
    pub access_token: Option<String>,
}

impl fmt::Debug for CreateSessionOAuthRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // todo: replace with `finish_non_exhaustive`
            Self::Google { .. } => f.debug_struct("Google").finish(),
        }
    }
}

make_path_parts!(DeleteSessionPath => "/v1/session");
