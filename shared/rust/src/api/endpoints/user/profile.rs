use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        session::NewSessionResponse,
        user::{PatchProfileRequest, PutProfileRequest, UserProfile},
    },
    error::EmptyError,
};

/// Fetch your own user profile.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = UserProfile;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Get;
}

/// Create or replace your user profile.
///
/// # Flow
///
/// # Errors
/// * Invalid request - [`400 - Bad Request`](http::StatusCode::BAD_REQUEST)
/// * Missing / bad auth - [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED)
/// * User not found - [`404 - Not Found`](http::StatusCode::NOT_FOUND)
/// * Profile image with ID not found - [`404 - Not Found`](http::StatusCode::NOT_FOUND)
/// * Taken username - [`409 - Conflict`](http::StatusCode::CONFLICT)
/// * Empty username - [`422 - Unprocessable Entity`](http::StatusCode::UNPROCESSABLE_ENTITY)
pub struct Put;
impl ApiEndpoint for Put {
    type Req = PutProfileRequest;
    type Res = NewSessionResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Put;
}

/// Update your user profile.
///
/// # Errors
///
/// * Invalid request - [`400 - Bad Request`](http::StatusCode::BAD_REQUEST)
/// * Missing / bad auth - [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED)
/// * Taken username - [`409 - Conflict`](http::StatusCode::CONFLICT)
/// * Empty username - [`422 - Unprocessable Entity`](http::StatusCode::UNPROCESSABLE_ENTITY)
pub struct Patch;
impl ApiEndpoint for Patch {
    type Req = PatchProfileRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Patch;
}
