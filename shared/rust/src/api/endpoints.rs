use serde::{de::DeserializeOwned, Serialize};

// todo: add some way of getting the method.
//  add something for path requests?
//  add something for auth required?

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
        user::{NoSuchUserError, UserProfile},
    };

    // POST
    pub struct Signin;
    impl ApiEndpoint for Signin {
        type Req = ();
        type Res = SigninSuccess;
        type Err = NoSuchUserError;
        const PATH: &'static str = "/v1/login";
    }

    // POST
    pub struct SingleSignOn;
    impl ApiEndpoint for SingleSignOn {
        type Req = ();
        type Res = SingleSignOnSuccess;
        type Err = ();
        const PATH: &'static str = "/v1/authorize";
    }

    // POST
    pub struct Register;
    impl ApiEndpoint for Register {
        type Req = RegisterRequest;
        type Res = RegisterSuccess;
        type Err = RegisterError;
        const PATH: &'static str = "/v1/user";
    }

    // GET
    pub struct Profile;
    impl ApiEndpoint for Profile {
        type Req = ();
        type Res = UserProfile;
        type Err = NoSuchUserError;
        const PATH: &'static str = "/v1/user/me/profile";
    }
}

pub mod category {
    use super::ApiEndpoint;
    use crate::category::{
        CategoryCreateError, CategoryDeleteError, CategoryGetError, CategoryResponse,
        CategoryUpdateError, CategoryUpdateRequest, CreateCategoryRequest, NewCategoryResponse,
    };

    // GET
    pub struct Get;
    impl ApiEndpoint for Get {
        type Req = ();
        type Res = CategoryResponse;
        type Err = CategoryGetError;
        const PATH: &'static str = "/v1/category";
    }

    // POST
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = CreateCategoryRequest;
        type Res = NewCategoryResponse;
        type Err = CategoryCreateError;
        const PATH: &'static str = "/v1/category";
    }

    // PATCH
    pub struct Update;
    impl ApiEndpoint for Update {
        type Req = CategoryUpdateRequest;
        type Res = ();
        type Err = CategoryUpdateError;
        const PATH: &'static str = "/v1/category/{id}";
    }

    // DELETE
    pub struct Delete;
    impl ApiEndpoint for Delete {
        type Req = ();
        type Res = ();
        type Err = CategoryDeleteError;
        const PATH: &'static str = "/v1/category/{id}";
    }
}
