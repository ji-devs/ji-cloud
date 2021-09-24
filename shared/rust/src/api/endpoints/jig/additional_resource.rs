use crate::{
    api::Method,
    domain::{
        jig::additional_resource::{
            AdditionalResourceCreateRequest, AdditionalResourceId, AdditionalResourceResponse,
            AdditionalResourceUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Get an additional resource by id.
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the additional resource or the parent jig doesn't exist.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = AdditionalResourceResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Get;
}

/// Add an additional resource URL to a JIG.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = AdditionalResourceCreateRequest;
    type Res = CreateResponse<AdditionalResourceId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/additional-resource";
    const METHOD: Method = Method::Post;
}

/// Update an additional resource URL for a JIG.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the additional resource or parent jig does not exist.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = AdditionalResourceUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Patch;
}

/// Delete an additional resource URL from a JIG
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the additional resource or parent jig does not exist.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Delete;
}
