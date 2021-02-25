use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        session::NewSessionResponse,
        user::{
            OtherUser, PutProfileRequest, UserLookupQuery, UserProfile, VerifyEmailRequest,
            VerifyEmailResponse,
        },
    },
    error::{auth::RegisterError, EmptyError},
};

/// Create or replace your User Profile.
///
/// Flow:
/// todo:
pub struct PutProfile;
impl ApiEndpoint for PutProfile {
    type Req = PutProfileRequest;
    type Res = NewSessionResponse;
    type Err = RegisterError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Put;
}

/// Register a new user
///
/// Flow:
/// 1. call this with `Authorization: Basic <email>:<password>`
///   & email gets sent to the user (204 response)
/// 2. call this *again* with `Authorization: Bearer <token>`
///   * recieve either a 200 (CreateSessionSuccess),
///     a 401 response (`<token>` is invalid),
///     or a 419 response (email is already in a registered account. This is okay because someone needs to have access to this email account anyway)
/// 4. call register with `X-CSRF` from the body of this response + cookie
pub struct Register;
impl ApiEndpoint for Register {
    type Req = ();
    type Res = ();
    type Err = RegisterError;
    const PATH: &'static str = "/v1/user";
    const METHOD: Method = Method::Post;
}

/// Verify a user's email
///
/// # Register Flow
/// 1. [`POST /v1/user`](Register)
///   * This will send the email for verification
/// 2. call this route with `Authorization: Bearer <token>` (`<token>` being the token from the email) and no body.
///   * recieve one of:
///     1. a 200 (`NewSessionResponse`)
///     2. a 401 response (`<token>` is invalid)
///
/// # Change email Flow (NOT CURRENTLY IMPLEMENTED)
/// 1. `PUT /v1/user/me/email`
///   * this will send an email to verify the *old* address
/// 2. Call this route with the token from the email
///   * this will send an email to verify the *new* address
/// 3. Call this route *again* with the token from the new address
///   * The new email will get set at this point, recieve one of:
///     1. a 204 (success!)
///     2. a 401 response (`token` is invalid)
///
/// # Resend verification email
/// 1. Call this route with no auth and `Resend`
///    * this resend will the verification email
///    * this will *always* return a 204 on success.
/// 2. Continue the flow you were in.
/// If no verification is in progress, no email will be sent.
pub struct VerifyEmail;
impl ApiEndpoint for VerifyEmail {
    type Req = VerifyEmailRequest;
    type Res = VerifyEmailResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/email/verify";
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
