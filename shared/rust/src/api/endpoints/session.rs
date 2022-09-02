use crate::{
    api::Method,
    domain::session::{
        CreateSessionOAuthPath, CreateSessionOAuthRequest, CreateSessionPath,
        CreateSessionResponse, DeleteSessionPath, GetOAuthPath, GetOAuthUrlResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Sign in.
///
/// requires `Basic` auth in the form `BASE64(email:password)`
/// see: <https://tools.ietf.org/html/rfc7617#section-2>
pub struct Create;
impl ApiEndpoint for Create {
    type Path = CreateSessionPath;
    type Req = ();
    type Res = CreateSessionResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Sign in via oauth.
///
/// Note: If the account doesn't exist, but the oauth token is valid, it'll return a token that can be used to create an account.
///
/// # Errors
/// (non exhaustive list)
/// If there is already a user with the oauth user's email,
/// and it isn't them - [`409 - Conflict`](http::StatusCode::CONFLICT)
///
/// # Flow (login)
/// 1. [`GET /v1/session/oauth/url/{service}/{kind}`](GetOAuthUrl)
/// 2. `POST /v1/session/oauth` (this route) with the token
///
/// # Flow (register)
/// 1. [`GET /v1/session/oauth/url/{service}/{kind}`](GetOAuthUrl)
///     * Returns the access token as a cookie + csrf token, in addition to any user profile info
///     given by the OAuth provider.
/// 2. `POST /v1/session/oauth` (this route) with the token
/// 3. [`POST /v1/user/me/profile`](crate::api::endpoints::user::CreateProfile)
///     * Optionally include the user profile information returned in #1
pub struct CreateOAuth;
impl ApiEndpoint for CreateOAuth {
    type Path = CreateSessionOAuthPath;
    type Req = CreateSessionOAuthRequest;
    type Res = CreateSessionResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get URL for oauth callback
///
/// # Flow (login/register)
/// 1. `GET /v1/session/oauth/url/{service}/{kind}` (this route)
/// 2. Continue from [`CreateOAuth`]
pub struct GetOAuthUrl;
impl ApiEndpoint for GetOAuthUrl {
    type Path = GetOAuthPath;
    type Req = ();
    type Res = GetOAuthUrlResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Delete a session (logout)
///
/// # Authorization
/// standard/any
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if the authorization is invalid
pub struct Delete;
impl ApiEndpoint for Delete {
    type Path = DeleteSessionPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
