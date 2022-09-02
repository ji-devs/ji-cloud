use super::ApiEndpoint;
use crate::{
    api::method::Method,
    domain::category::{
        CategoryResponse, CreateCategoryPath, CreateCategoryRequest, DeleteCategoryPath,
        GetCategoryPath, GetCategoryRequest, NewCategoryResponse, UpdateCategoryPath,
        UpdateCategoryRequest,
    },
    error::EmptyError,
};

/// Get a tree of categories.
///
/// # Authorization
/// No authorization required.
pub struct Get;
impl ApiEndpoint for Get {
    type Path = GetCategoryPath;
    type Req = GetCategoryRequest;
    type Res = CategoryResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Create a category.
///
/// # Authorization
/// Standard + [`UserScope::ManageCategory`](crate::domain::user::UserScope).
pub struct Create;
impl ApiEndpoint for Create {
    type Path = CreateCategoryPath;
    type Req = CreateCategoryRequest;
    type Res = NewCategoryResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Update a category.
///
/// # Authorization
/// Standard + [`UserScope::ManageCategory`](crate::domain::user::UserScope).
pub struct Update;
impl ApiEndpoint for Update {
    type Path = UpdateCategoryPath;
    type Req = UpdateCategoryRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete a category.
///
/// # Authorization
/// Standard + [`UserScope::ManageCategory`](crate::domain::user::UserScope).
pub struct Delete;
impl ApiEndpoint for Delete {
    type Path = DeleteCategoryPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
