use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        session::NewSessionResponse,
        user::{
            ChangePasswordPath, ChangePasswordRequest, CreateUserPath, CreateUserRequest,
            OtherUser, ResetEmailPath, ResetEmailRequest, ResetEmailResponse, ResetPasswordPath,
            ResetPasswordRequest, UserBrowsePath, UserBrowseQuery, UserBrowseResponse,
            UserDeletePath, UserLookupPath, UserLookupQuery, UserSearchPath, UserSearchQuery,
            UserSearchResponse, VerifyEmailPath, VerifyEmailRequest, VerifyResetEmailPath,
            VerifyResetEmailRequest,
        },
    },
    error::EmptyError,
};

mod colors;
mod fonts;
mod profile;
mod public_user;

pub use colors::{
    Create as CreateColor, Delete as DeleteColor, Get as GetColors, Update as UpdateColor,
};

pub use fonts::{
    Create as CreateFont, Delete as DeleteFont, Get as GetFonts, Update as UpdateFont,
};

pub use profile::{Create as CreateProfile, Get as Profile, Patch as PatchProfile};

pub use public_user::{
    BrowseFollowers, BrowseFollowing, BrowsePublicUser, BrowseUserCourses as BrowseCourses,
    BrowseUserJigs, BrowseUserResources as BrowseResources, Follow, Get as GetPublicUser,
    SearchPublicUser as Search, Unfollow,
};

/// Create a new user.
///
/// # Flow
/// 1. `POST` to this route
///     * recieve one of:
///         1. email gets sent to the user - [`204 - No Content`](http::StatusCode::NO_CONTENT)
///         2. email already exists - [`409 - Conflict`](http::StatusCode::CONFLICT)
///             * In the future this may contain information about *how* the email is registered.
/// 2. [`POST /v1/user/verify-email`](VerifyEmail)
/// 3. [`POST /v1/user/me/profile`](profile::Create)
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateUserRequest;
    type Res = ();
    type Path = CreateUserPath;
    type Err = EmptyError;
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
    type Path = VerifyEmailPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// verifies the updated email for a user
pub struct VerifyResetEmail;
impl ApiEndpoint for VerifyResetEmail {
    type Req = VerifyResetEmailRequest;
    type Res = ();
    type Path = VerifyResetEmailPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Resets user email
///
/// # Flow
/// 1. `POST` to this route
///     * recieve one of:
///         1. email gets sent to the user - [`204 - No Content`](http::StatusCode::NO_CONTENT)
///         2. email already exists - [`409 - Conflict`](http::StatusCode::CONFLICT)
/// 2. [`PATCH /v1/user/reset-email`](ResetEmail)
pub struct ResetEmail;
impl ApiEndpoint for ResetEmail {
    type Req = ResetEmailRequest;
    type Res = ResetEmailResponse;
    type Path = ResetEmailPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
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
    type Path = ResetPasswordPath;
    type Err = EmptyError;
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
    type Path = ChangePasswordPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Find a user by username.
pub struct UserLookup;
impl ApiEndpoint for UserLookup {
    type Req = UserLookupQuery;
    type Res = OtherUser;
    type Path = UserLookupPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Delete your account.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = UserDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Browse for users.
///
/// # Errors
///
/// If the user isn't an Admin ([`403 - Forbidden`](http::StatusCode::FORBIDDEN))
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = UserBrowseQuery;
    type Res = UserBrowseResponse;
    type Path = UserBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Browse for users.
///
/// # Errors
///
/// If the user isn't an Admin ([`403 - Forbidden`](http::StatusCode::FORBIDDEN))
pub struct SearchUser;
impl ApiEndpoint for SearchUser {
    type Req = UserSearchQuery;
    type Res = UserSearchResponse;
    type Path = UserSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
