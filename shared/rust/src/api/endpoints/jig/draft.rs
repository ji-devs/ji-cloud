use super::super::ApiEndpoint;

use crate::{
    api::Method,
    domain::{
        jig::{JigDraftResponse, JigId},
        CreateResponse,
    },
    error::EmptyError,
};

/// Create an draft of a JIG (unpublished clone of a published JIG).
///
/// Returns the id of the draft jig, which can be updated through the jig APIs.
/// The fields `publish_at` and `is_public` are ignored by drafts. Republishing keeps the original live jig's values for these fields.
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
///
/// ['NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
///
/// ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the jig is a draft.
///
/// ['Conflict'](http::StatusCode::CONFLICT) if a draft already exists for this jig.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ();
    type Res = CreateResponse<JigId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Post;
}

/// Get the id of the draft jig associated with the jig.
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
///
/// ['NotFound'](http::StatusCode::NOT_FOUND) if the jig or a draft does not exist.
///
/// ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the jig is a draft.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = JigDraftResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Get;
}

/// Move-publish a draft to the live jig.
///
/// This deletes the draft jig.
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
///
/// ['NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
///
/// ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the jig is a draft.
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Put;
}
