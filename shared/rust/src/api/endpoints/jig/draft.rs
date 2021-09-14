use super::super::ApiEndpoint;

use crate::{api::Method, domain::jig::JigIdResponse, error::EmptyError};

/// Publishes a draft jig to the live jig.
///
/// This replaces the contents of the live jig with that of the draft.
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the jig is a draft.
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Put;
}

/// Fetches the id of the draft jig for a live jig. If called on a draft jig, it will return status `422 UNPROCESSABLE_ENTITY`.
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the draft does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the jig is a draft.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = JigIdResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Get;
}

/// Fetches the id of the live jig for a jig. If called on a live jig, it will return status `422 UNPROCESSABLE_ENTITY`.
///
/// # Errors FIXME
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the jig or a draft does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the jig is live.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = JigIdResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/live";
    const METHOD: Method = Method::Get;
}
