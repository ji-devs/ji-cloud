use crate::{
    api::Method,
    domain::{
        jig::module::{ModuleCreateRequest, ModuleId, ModuleResponse, ModuleUpdateRequest},
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Get a Module by index.
///
/// # Authorization
/// Standard
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent jig doesn't exist.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = ModuleResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/module/{module_id}";
    const METHOD: Method = Method::Get;
}

/// Create a Module.
///
/// # Authorization
/// Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ModuleCreateRequest;
    type Res = CreateResponse<ModuleId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/module";
    const METHOD: Method = Method::Post;
}

/// Update a Module.
///
/// # Authorization
/// Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid) or the request is missing/invalid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the jig or module does not exist.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = ModuleUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/module/{module_id}";
    const METHOD: Method = Method::Patch;
}

/// Delete a Module.
///
/// # Authorization
/// Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the jig or module does not exist.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the given `id` is not a [`Uuid`](uuid::Uuid).
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/module/{module_id}";
    const METHOD: Method = Method::Delete;
}
