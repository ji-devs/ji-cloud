use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        session::NewSessionResponse,
        user::{
            CreateUserRequest, OtherUser, PutProfileRequest, ResetPasswordRequest, UserLookupQuery,
            UserProfile, VerifyEmailRequest, VerifyEmailResponse,
        },
    },
    error::EmptyError,
};

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
pub struct PutProfile;
impl ApiEndpoint for PutProfile {
    type Req = PutProfileRequest;
    type Res = NewSessionResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Put;
}

/// Create a new user
///
/// # Flow
/// 1. `POST` to this route
///     * recieve one of:
///         1. email gets sent to the user - [`204 - No Content`](http::StatusCode::NO_CONTENT)
///         2. email already exists - [`409 - Conflict`](http::StatusCode::CONFLICT)
///             * In the future this may contain information about *how* the email is registered.
/// 2. [`POST /v1/user/verify-email`](VerifyEmail)
/// 3. [`PUT /v1/user/me/profile`](PutProfile)
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateUserRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user";
    const METHOD: Method = Method::Post;
}

/// Verify a user's email
///
/// # Register Flow
/// 1. [`POST /v1/user`](Create)
///     * This will send the email for verification
/// 2. `POST` this route with `Authorization: Bearer <token>` (`<token>` being the token from the email) and no body.
///     * recieve one of:
///         1. a 200 ([`NewSessionResponse`])
///         2. a 401 response (`<token>` is invalid)
///
/// # Change email Flow (NOT CURRENTLY IMPLEMENTED)
/// 1. `PUT /v1/user/me/email`
///     * this will send an email to verify the *old* address
/// 2. `POST` this route with the token from the email
///     * this will send an email to verify the *new* address
/// 3. `POST` this route *again* with the token from the new address
///     * The new email will get set at this point, recieve one of:
///         1. a 204 (success!)
///         2. a 401 response (`token` is invalid)
///
/// # Resend verification email
/// 1. `POST` this route with no auth and [`Resend`](VerifyEmailRequest::Resend)
///     * this resend will the verification email
///     * this will *always* return a 204 on success.
/// 2. Continue the flow you were in.
///
/// If no verification is in progress, no email will be sent.
pub struct VerifyEmail;
impl ApiEndpoint for VerifyEmail {
    type Req = VerifyEmailRequest;
    type Res = VerifyEmailResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/verify-email";
    const METHOD: Method = Method::Post;
}

/// Reset a user's password
///
/// # Flow (NOT IMPLEMENTED)
/// 1. `POST` This route.
///      * email gets sent to the included email address
///      * recieve [`204 - No Content`](http::StatusCode::NO_CONTENT)
/// 2. `PUT /v1/user/me/password`
pub struct ResetPassword;
impl ApiEndpoint for ResetPassword {
    type Req = ResetPasswordRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/password-reset";
    const METHOD: Method = Method::Post;
}

/// Fetch your own user profile.
pub struct Profile;
impl ApiEndpoint for Profile {
    type Req = ();
    type Res = UserProfile;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Get;
}

/// Find a user by username.
pub struct UserLookup;
impl ApiEndpoint for UserLookup {
    type Req = UserLookupQuery;
    type Res = OtherUser;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user";
    const METHOD: Method = Method::Get;
}
