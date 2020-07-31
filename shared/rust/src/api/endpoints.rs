use serde::{de::DeserializeOwned, Serialize};

// todo: add some way of getting the method.
pub trait ApiEndpoint {
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;
    type Err: DeserializeOwned + Serialize;
    const PATH: &'static str;
}

pub mod user {
    use super::ApiEndpoint;

    use crate::{
        auth::{
            RegisterError, RegisterRequest, RegisterSuccess, SigninSuccess, SingleSignOnSuccess,
        },
        user::{NoSuchUserError, User},
    };

    pub struct Signin;
    impl ApiEndpoint for Signin {
        type Req = ();
        type Res = SigninSuccess;
        type Err = NoSuchUserError;
        const PATH: &'static str = "/v1/login";
    }

    pub struct SingleSignOn;
    impl ApiEndpoint for SingleSignOn {
        type Req = ();
        type Res = SingleSignOnSuccess;
        type Err = ();
        const PATH: &'static str = "/v1/authorize";
    }

    pub struct Register;
    impl ApiEndpoint for Register {
        type Req = RegisterRequest;
        type Res = RegisterSuccess;
        type Err = RegisterError;
        const PATH: &'static str = "/v1/user";
    }

    pub struct Profile;
    impl ApiEndpoint for Profile {
        type Req = ();
        type Res = User;
        type Err = NoSuchUserError;
        const PATH: &'static str = "/v1/user/me/profile";
    }
}
