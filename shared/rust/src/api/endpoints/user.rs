use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        session::NewSessionResponse,
        user::{
            ChangePasswordRequest, CreateUserRequest, OtherUser, ResetPasswordRequest,
            UserLookupQuery, VerifyEmailRequest,
        },
    },
    error::EmptyError,
};

mod colors;
mod fonts;
mod profile;

pub use colors::{
    Create as CreateColor, Delete as DeleteColor, Get as GetColors, Update as UpdateColor,
};

pub use fonts::{
    Create as CreateFont, Delete as DeleteFont, Get as GetFonts, Update as UpdateFont,
};

pub use profile::{Create as CreateProfile, Get as Profile, Patch as PatchProfile};

/// Create a new user.
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

/// Verify a user's email.
///
/// # Register Flow
/// 1. [`POST /v1/user`](Create)
///     * This will send the email for verification
/// 2. `POST` this route with the Verify request.
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
    type Res = Option<NewSessionResponse>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/verify-email";
    const METHOD: Method = Method::Post;
}

/// Reset a user's password.
///
/// # Flow
/// 1. `POST` This route.
///      * email gets sent to the included email address
///      * recieve [`204 - No Content`](http::StatusCode::NO_CONTENT)
/// 2. [`PUT /v1/user/me/password`](ChangePassword)
pub struct ResetPassword;
impl ApiEndpoint for ResetPassword {
    type Req = ResetPasswordRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/password-reset";
    const METHOD: Method = Method::Post;
}

/// Change your password.
///
/// # Responses
///
/// success - [`204 - No Content`](http::StatusCode::NO_CONTENT)
///
/// # Errors
///
/// If the user isn't authorized to change their password ([`403 - Forbidden`](http::StatusCode::FORBIDDEN))
pub struct ChangePassword;
impl ApiEndpoint for ChangePassword {
    type Req = ChangePasswordRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/password";
    const METHOD: Method = Method::Put;
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

/// Delete your account.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me";
    const METHOD: Method = Method::Delete;
}
