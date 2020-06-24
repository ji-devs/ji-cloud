use serde::{de::DeserializeOwned, Serialize};
use super::result::ResultResponse;


pub trait ApiEndpoint {
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;
    type Err: DeserializeOwned + Serialize;

    fn endpoint_str() -> &'static str;
 
    #[cfg(feature = "frontend")]
    fn url() -> String {
        crate::frontend::path::api_url(Self::endpoint_str())
    }
}

pub mod user {
    use super::ApiEndpoint;
    #[cfg(feature = "frontend")]
    use crate::frontend::{
        path,
        fetch::{api_with_auth_unwrap, api_with_token_unwrap}
    };

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
        
        fn endpoint_str() -> &'static str {
            "/user/signin"
        }
    }

    #[cfg(feature = "frontend")]
    impl Signin {
        pub async fn fetch(token:&str) -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_token_unwrap::< _, _, ()>(&Self::url(), token, None).await
        }
    }

    // single sign on
    pub struct SingleSignOn { }

    impl ApiEndpoint for SingleSignOn {
        type Req = ();
        type Res = SingleSignOnSuccess;
        type Err = ();
        
        fn endpoint_str() -> &'static str {
            "/user/single-sign-on"
        }
    }

    #[cfg(feature = "frontend")]
    impl SingleSignOn {
        pub async fn fetch(token:&str) -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_token_unwrap::< _, _, ()>(&Self::url(), token, None).await
        }
    }

    // register
    pub struct Register { }

    impl ApiEndpoint for Register {
        type Req = RegisterRequest;
        type Res = RegisterSuccess;
        type Err = RegisterError;
        
        fn endpoint_str() -> &'static str {
            "/user/register"
        }
    }

    #[cfg(feature = "frontend")]
    impl Register {
        pub async fn fetch(token:&str, req:&<Self as ApiEndpoint>::Req) -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_token_unwrap(&Self::url(), token, Some(req)).await
        }
    }

    // profile
    
    pub struct Profile { }

    impl ApiEndpoint for Profile {
        type Req = ();
        type Res = User;
        type Err = NoSuchUserError;
        
        fn endpoint_str() -> &'static str {
            "user/profile"
        }
    }

    #[cfg(feature = "frontend")]
    impl Profile {
        pub async fn fetch() -> Result < <Self as ApiEndpoint>::Res, <Self as ApiEndpoint>::Err> {
            api_with_auth_unwrap::< _, _, ()>(&Self::url(), None).await
        }
    }
}

