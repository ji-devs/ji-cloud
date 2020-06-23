use serde::{de::DeserializeOwned, Serialize};
use super::result::ResultResponse;

pub enum ApiMethod {
    Get,
    Post
}

pub trait ApiEndpoint {
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;
    type Err: DeserializeOwned + Serialize;

    fn uri() -> &'static str;
    fn method() -> ApiMethod;

}

pub mod user {
    use super::{ApiEndpoint, ApiMethod};
    #[cfg(feature = "frontend")]
    use crate::frontend::fetch::{api_with_auth_unwrap, api_with_token_unwrap};

    use crate::{
        auth::{SigninSuccess, RegisterRequest, RegisterSuccess, RegisterError, SingleSignOnSuccess},
        user::{UserRole, User, NoSuchUserError},
    };

    // signin
    pub struct Signin { }

    impl ApiEndpoint for Signin {
        type Req = ();
        type Res = SigninSuccess;
        type Err = ();
        
        fn uri() -> &'static str {
            "/user/signin"
        }

        fn method() -> ApiMethod {
            ApiMethod::Get 
        }
    }

    #[cfg(feature = "frontend")]
    impl Signin {
        pub async fn fetch(token:&str) -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_token_unwrap::< _, _, ()>(Self::uri(), token, None).await
        }
    }

    // single sign on
    pub struct SingleSignOn { }

    impl ApiEndpoint for SingleSignOn {
        type Req = ();
        type Res = SingleSignOnSuccess;
        type Err = ();
        
        fn uri() -> &'static str {
            "/user/single-sign-on"
        }

        fn method() -> ApiMethod {
            ApiMethod::Get 
        }
    }

    #[cfg(feature = "frontend")]
    impl SingleSignOn {
        pub async fn fetch(token:&str) -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_token_unwrap::< _, _, ()>(Self::uri(), token, None).await
        }
    }

    // register
    pub struct Register { }

    impl ApiEndpoint for Register {
        type Req = RegisterRequest;
        type Res = RegisterSuccess;
        type Err = RegisterError;
        
        fn uri() -> &'static str {
            "/user/register"
        }

        fn method() -> ApiMethod {
            ApiMethod::Get 
        }
    }

    #[cfg(feature = "frontend")]
    impl Register {
        pub async fn fetch(token:&str, req:&<Self as ApiEndpoint>::Req) -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_token_unwrap(Self::uri(), token, Some(req)).await
        }
    }

    // profile
    
    pub struct Profile { }

    impl ApiEndpoint for Profile {
        type Req = ();
        type Res = User;
        type Err = NoSuchUserError;
        
        fn uri() -> &'static str {
            "/user/profile"
        }

        fn method() -> ApiMethod {
            ApiMethod::Get 
        }
    }

    #[cfg(feature = "frontend")]
    impl Profile {
        pub async fn fetch() -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_auth_unwrap::< _, _, ()>(Self::uri(), None).await
        }
    }
}

