use crate::{
    api::Method,
    domain::{
        additional_resource::{
            AdditionalResource, AdditionalResourceCreateRequest, AdditionalResourceId,
            AdditionalResourceUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Get an additional resource on a draft jig copy by id.
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the additional resource or the parent jig doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = AdditionalResource;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Get;
}

/// Get an additional resource on a draft jig copy by id.
///
/// # Authorization
/// Standard
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the additional resource or the parent jig doesn't exist.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = AdditionalResource;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/live/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Get;
}

/// Add an additional resource URL to a draft JIG.
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
    const PATH: &'static str = "/v1/jig/{id}/draft/additional-resource";
    const METHOD: Method = Method::Post;
}

/// Update an additional resources to a draft JIG.
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
pub struct Update;
impl ApiEndpoint for Update {
    type Req = AdditionalResourceUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Patch;
}

/// Delete an additional resource URL from a draft JIG.
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
    const PATH: &'static str = "/v1/jig/{id}/draft/additional-resource/{additional_resource_id}";
    const METHOD: Method = Method::Delete;
}
