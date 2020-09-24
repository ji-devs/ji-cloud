use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        auth::{RegisterRequest, RegisterSuccess, SigninSuccess, SingleSignOnSuccess},
        user::OtherUser,
        user::UserProfile,
    },
    error::auth::RegisterError,
    error::user::NoSuchUserError,
};

/// Sign in.
pub struct Signin;
impl ApiEndpoint for Signin {
    type Req = ();
    type Res = SigninSuccess;
    type Err = NoSuchUserError;
    const PATH: &'static str = "/v1/login";
    const METHOD: Method = Method::Post;
}

/// Sign in via SSO.
pub struct SingleSignOn;
impl ApiEndpoint for SingleSignOn {
    type Req = ();
    type Res = SingleSignOnSuccess;
    type Err = ();
    const PATH: &'static str = "/v1/authorize";
    const METHOD: Method = Method::Post;
}

/// Register a new user.
pub struct Register;
impl ApiEndpoint for Register {
    type Req = RegisterRequest;
    type Res = RegisterSuccess;
    type Err = RegisterError;
    const PATH: &'static str = "/v1/user";
    const METHOD: Method = Method::Post;
}

/// Fetch a user's profile.
pub struct Profile;
impl ApiEndpoint for Profile {
    type Req = ();
    type Res = UserProfile;
    type Err = NoSuchUserError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Get;
}

/// Find a user by username.
pub struct UserByName;
impl ApiEndpoint for UserByName {
    type Req = ();
    type Res = OtherUser;
    type Err = NoSuchUserError;
    const PATH: &'static str = "/v1/user/{name}";
    const METHOD: Method = Method::Get;
}
