use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::{
        auth::RegisterRequest,
        session::CreateSessionSuccess,
        user::{OtherUser, UserLookupQuery, UserProfile},
    },
    error::{auth::RegisterError, EmptyError},
};

/// Register a new user.
pub struct Register;
impl ApiEndpoint for Register {
    type Req = RegisterRequest;
    type Res = CreateSessionSuccess;
    type Err = RegisterError;
    const PATH: &'static str = "/v1/user";
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
