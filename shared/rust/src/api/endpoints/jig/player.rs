use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::jig::player::{JigPlayerSession, JigPlayerSessionCode, JigPlayerSessionCreateRequest, JigPlayerSessionToken, JigPlayerSessionCompleteRequest, JigPlayCount, JigPlayerSessionCreateRequestForPlayer},
    error::EmptyError,
};

/// Create a player session from a jig. Requestor needs permissions over the jig.
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
/// * ['409 - Conflict'](http::StatusCode::CONFLICT) if a code already exists for this jig.
/// * [`503 - ServiceUnavailable`](http::StatusCode::SERVICE_UNAVAILABLE) if some how we have reached the maximum number of possible session codes.
///
/// # Note
///
/// The code is computed with the following procedure:
///
/// 0. Converting the jig's UUID into a slice of 16 bytes, or 8 words.
/// 1. XOR all eight words together into an accumulator `acc`.
/// 2. Clamping to within the digit range requirement. For n = 4 digits with a range of `[0000, 9999]`,
/// this is done as `code = abs(acc % 10000)`.
/// 3. If the code is unique, stop.
///     * Else there already exists a different player session with the same code. Rehash as `code_(i+1) = clamp(code_i + 2.pow(i))` where `i` is the # of rehash attempts.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = JigPlayerSessionCreateRequest;
    type Res = JigPlayerSessionCode;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/player";
    const METHOD: Method = Method::Post;
}

/// Create a session for a user who isn't the author
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
pub struct CreatePlayerSession;
impl ApiEndpoint for CreatePlayerSession {
    type Req = JigPlayerSessionCreateRequestForPlayer;
    type Res = JigPlayerSessionToken;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/player/instance";
    const METHOD: Method = Method::Post;
}

/// Complete a session for a user who isn't the author and update the jig play count
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
pub struct CompletePlayerSession;
impl ApiEndpoint for CompletePlayerSession {
    type Req = JigPlayerSessionCompleteRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/player/instance/complete";
    const METHOD: Method = Method::Post;
}

/// Get a player session given it's code/index.
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = JigPlayerSession;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/player/{index}";
    const METHOD: Method = Method::Get;
}

/// Fetch the player session code/index.
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
/// * ['409 - Conflict'](http::StatusCode::CONFLICT) if a code already exists for this jig.
pub struct GetPlayerSessionCode;
impl ApiEndpoint for GetPlayerSessionCode {
    type Req = ();
    type Res = JigPlayerSessionCode;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/player";
    const METHOD: Method = Method::Get;
}
