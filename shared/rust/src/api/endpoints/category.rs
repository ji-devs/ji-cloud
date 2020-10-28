use super::ApiEndpoint;
use crate::{
    api::method::Method,
    domain::category::{
        CategoryResponse, CreateCategoryRequest, GetCategoryRequest, NewCategoryResponse,
        UpdateCategoryRequest,
    },
    error::{
        category::{CreateError, UpdateError},
        DeleteError, GetError,
    },
};

/// Get a tree of categories.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = GetCategoryRequest;
    type Res = CategoryResponse;
    type Err = GetError;
    const PATH: &'static str = "/v1/category";
    const METHOD: Method = Method::Get;
}

/// Create a category.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateCategoryRequest;
    type Res = NewCategoryResponse;
    type Err = CreateError;
    const PATH: &'static str = "/v1/category";
    const METHOD: Method = Method::Post;
}

/// Update a category.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = UpdateCategoryRequest;
    type Res = ();
    type Err = UpdateError;
    const PATH: &'static str = "/v1/category/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete a category.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = DeleteError;
    const PATH: &'static str = "/v1/category/{id}";
    const METHOD: Method = Method::Delete;
}
