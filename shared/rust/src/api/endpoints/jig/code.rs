use crate::{
    api::Method,
    domain::jig::code::{JigCodeResponse, JigIdFromCodeRequest, JigIdFromCodeResponse},
    error::EmptyError,
};

use super::ApiEndpoint;

/// Create a short code hash for the jig.
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
/// * ['Conflict'](http::StatusCode::CONFLICT) if a code already exists for this jig.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ();
    type Res = JigCodeResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/code";
    const METHOD: Method = Method::Post;
}

/// Get the code associated with the jig.
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if a code does not exist for this jig.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = JigCodeResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/code";
    const METHOD: Method = Method::Get;
}

/// Get the ID of the jig associated with this code.
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if a jig does not exist for this code.
pub struct GetJig;
impl ApiEndpoint for GetJig {
    type Req = JigIdFromCodeRequest;
    type Res = JigIdFromCodeResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/code";
    const METHOD: Method = Method::Get;
}
