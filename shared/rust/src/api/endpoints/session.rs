use crate::{
    api::Method,
    domain::session::{
        CreateSessionOAuthRequest, CreateSessionOAuthResponse, CreateSessionSuccess,
        GetOAuthUrlResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Sign in.
/// requires `Basic` auth in the form `BASE64(email:password)`
/// see: https://tools.ietf.org/html/rfc7617#section-2
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ();
    type Res = CreateSessionSuccess;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session";
    const METHOD: Method = Method::Post;
}

/// Sign in via oauth
/// Note: If the account doesn't exist, but the oauth token is valid, it'll return a token that can be used to create an account.
pub struct CreateOAuth;
impl ApiEndpoint for CreateOAuth {
    type Req = CreateSessionOAuthRequest;
    type Res = CreateSessionOAuthResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session/oauth";
    const METHOD: Method = Method::Post;
}

/// Get URL for oauth callback
pub struct GetOAuthUrl;
impl ApiEndpoint for GetOAuthUrl {
    type Req = ();
    type Res = GetOAuthUrlResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/session/oauth/url/{service}/{kind}";
    const METHOD: Method = Method::Get;
}
