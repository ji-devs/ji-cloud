use crate::api::method::Method;
use serde::{de::DeserializeOwned, Serialize};

//  add something for path requests?
//  add something for auth required?

pub trait ApiEndpoint {
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;
    type Err: DeserializeOwned + Serialize;
    const PATH: &'static str;
    const METHOD: Method;
}

pub mod user {
    use super::ApiEndpoint;

    use crate::{
        api::method::Method,
        auth::{
            RegisterError, RegisterRequest, RegisterSuccess, SigninSuccess, SingleSignOnSuccess,
        },
        user::{NoSuchUserError, UserProfile},
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
}

pub mod category {
    use super::ApiEndpoint;
    use crate::{
        api::method::Method,
        category::{
            CategoryCreateError, CategoryDeleteError, CategoryGetError, CategoryResponse,
            CategoryUpdateError, CreateCategoryRequest, GetCategoryInverseTreeRequest,
            NewCategoryResponse, UpdateCategoryRequest,
        },
    };

    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = CategoryResponse;
        type Err = CategoryGetError;
        const PATH: &'static str = "/v1/category";
        const METHOD: Method = Method::Get;
    }

    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = CreateCategoryRequest;
        type Res = NewCategoryResponse;
        type Err = CategoryCreateError;
        const PATH: &'static str = "/v1/category";
        const METHOD: Method = Method::Post;
    }

    pub struct GetInverse;
    impl ApiEndpoint for GetInverse {
        type Req = GetCategoryInverseTreeRequest;
        type Res = CategoryResponse;
        type Err = CategoryGetError;
        const PATH: &'static str = "/v1/category/inverse";
        const METHOD: Method = Method::Get;
    }

    pub struct Update;
    impl ApiEndpoint for Update {
        type Req = UpdateCategoryRequest;
        type Res = ();
        type Err = CategoryUpdateError;
        const PATH: &'static str = "/v1/category/{id}";
        const METHOD: Method = Method::Patch;
    }

    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = CategoryDeleteError;
        const PATH: &'static str = "/v1/category/{id}";
        const METHOD: Method = Method::Delete;
    }
}
