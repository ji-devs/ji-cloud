use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        auth::{RegisterRequest, RegisterSuccess, SigninSuccess, SingleSignOnSuccess},
        user::{NoSuchUserError, UserProfile},
    },
    error::auth::RegisterError,
};

pub struct Signin;
impl ApiEndpoint for Signin {
    type Req = ();
    type Res = SigninSuccess;
    type Err = NoSuchUserError;
    const PATH: &'static str = "/v1/login";
    const METHOD: Method = Method::Post;
}

pub struct SingleSignOn;
impl ApiEndpoint for SingleSignOn {
    type Req = ();
    type Res = SingleSignOnSuccess;
    type Err = ();
    const PATH: &'static str = "/v1/authorize";
    const METHOD: Method = Method::Post;
}

pub struct Register;
impl ApiEndpoint for Register {
    type Req = RegisterRequest;
    type Res = RegisterSuccess;
    type Err = RegisterError;
    const PATH: &'static str = "/v1/user";
    const METHOD: Method = Method::Post;
}

pub struct Profile;
impl ApiEndpoint for Profile {
    type Req = ();
    type Res = UserProfile;
    type Err = NoSuchUserError;
    const PATH: &'static str = "/v1/user/me/profile";
    const METHOD: Method = Method::Get;
}
