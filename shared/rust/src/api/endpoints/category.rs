use super::ApiEndpoint;
use crate::{
    api::method::Method,
    domain::category::{
        CategoryResponse, CreateCategoryRequest, GetCategoryRequest, NewCategoryResponse,
        UpdateCategoryRequest,
    },
    error::category::{
        CategoryCreateError, CategoryDeleteError, CategoryGetError, CategoryUpdateError,
    },
};

pub struct Get;
impl ApiEndpoint for Get {
    type Req = GetCategoryRequest;
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
