use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        session::NewSessionResponse,
        user::{PutProfileRequest, UserProfile},
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

/// Create or replace your User Profile.
///
/// Note: Currently this can only be done at signup time,
/// in the future `PATCH /v1/user/me/profile` will exist to handle editing your profile.
/// # Flow
///
/// # Errors
/// * Invalid request - [`400 - Bad Request`](http::StatusCode::BAD_REQUEST)
/// * Missing / bad auth - [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED)
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
