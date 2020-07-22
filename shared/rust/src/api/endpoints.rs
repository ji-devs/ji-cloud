use serde::{de::DeserializeOwned, Serialize};
use super::result::ResultResponse;


pub trait ApiEndpoint {
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;
    type Err: DeserializeOwned + Serialize;
}

pub mod user {
    use super::ApiEndpoint;

    use crate::{
        auth::{SigninSuccess, RegisterRequest, RegisterSuccess, RegisterError, SingleSignOnSuccess},
        user::{UserRole, User, NoSuchUserError},
    };

    pub struct Signin { }
    impl ApiEndpoint for Signin {
        type Req = ();
        type Res = SigninSuccess;
        type Err = NoSuchUserError;
    }


    pub struct SingleSignOn { }
    impl ApiEndpoint for SingleSignOn {
        type Req = ();
        type Res = SingleSignOnSuccess;
        type Err = ();
    }


    pub struct Register { }
    impl ApiEndpoint for Register {
        type Req = RegisterRequest;
        type Res = RegisterSuccess;
        type Err = RegisterError;
    }

    pub struct Profile { }
    impl ApiEndpoint for Profile {
        type Req = ();
        type Res = User;
        type Err = NoSuchUserError;
    }
}

