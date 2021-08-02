//! Routes to manage image tags

use super::super::ApiEndpoint;
use crate::{
    api::Method,
    domain::image::tag::{
        ImageTagCreateRequest, ImageTagListResponse, ImageTagResponse, ImageTagUpdateRequest,
    },
    error::EmptyError,
};

/// List all image tags.
///
/// # Authorization
/// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
pub struct List;

impl ApiEndpoint for List {
    type Req = ();
    type Res = ImageTagListResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/tag/all";
    const METHOD: Method = Method::Get;
}

/// Create an image tag.
///
/// # Authorization
/// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
///
/// [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
///
/// [`Conflict`](http::StatusCode::CONFLICT) if the requested `index` is already occupied.
pub struct Create;

impl ApiEndpoint for Create {
    type Req = ImageTagCreateRequest;
    type Res = ImageTagResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/tag/{index}";
    const METHOD: Method = Method::Post;
}

/// Update an image tag by index.
///
/// # Authorization
/// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image tag does not exist.
/// * [`Conflict`](http::StatusCode::CONFLICT) if the requested `index` is already occupied.
pub struct Update;

impl ApiEndpoint for Update {
    type Req = ImageTagUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/tag/{index}";
    const METHOD: Method = Method::Patch;
}

/// Delete an image tag by index.
///
/// # Authorization
/// Standard + [`UserScope::Admin`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the image tag does not exist.
pub struct Delete;

impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/image/tag/{index}";
    const METHOD: Method = Method::Delete;
}
