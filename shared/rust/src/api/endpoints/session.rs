use crate::{
    api::Method,
    domain::session::{CreateSessionOAuthRequest, CreateSessionResponse, GetOAuthUrlResponse},
    error::EmptyError,
};

use super::ApiEndpoint;

/// Sign in.
///
/// requires `Basic` auth in the form `BASE64(email:password)`
/// see: <https://tools.ietf.org/html/rfc7617#section-2>
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ();
    type Res = CreateSessionResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session";
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
/// 2. `POST /v1/session/oauth` (this route) with the token
/// 3. [`PUT /v1/user/me/profile`](super::user::PutProfile)
pub struct CreateOAuth;
impl ApiEndpoint for CreateOAuth {
    type Req = CreateSessionOAuthRequest;
    type Res = CreateSessionResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session/oauth";
    const METHOD: Method = Method::Post;
}

/// Get URL for oauth callback
///
/// # Flow (login/register)
/// 1. `GET /v1/session/oauth/url/{service}/{kind}` (this route)
/// 2. Continue from [`CreateOAuth`]
pub struct GetOAuthUrl;
impl ApiEndpoint for GetOAuthUrl {
    type Req = ();
    type Res = GetOAuthUrlResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session/oauth/url/{service}/{kind}";
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
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session";
    const METHOD: Method = Method::Delete;
}
