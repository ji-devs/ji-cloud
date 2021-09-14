use crate::{
    api::{ApiEndpoint, Method},
    domain::jig::player::{
        JigPlayerSessionCreateRequest, JigPlayerSessionCreateResponse, JigPlayerSessionListResponse,
    },
    error::EmptyError,
};

/// Create a player session from a jig. Requestor needs permissions over the jig.
///
/// # Flow
///
/// 1. Author/admin creates a player session using [`POST /v1/jig/player`](Create)
///     a. This is represented by a *session code/index*
/// 2. Unauthed user instantiates the player session. This creates an instance of a session. [`POST /v1/jig/player/session`](instance::Create) returns:
///     a. A short lived token, which identifies the user and the session instance.
///     b. The player session settings.
///     c. `JigId` of the JIG on which the session was created.
/// 3. Unauthed user posts short lived token to complete the instance. [`POST /v1/jig/player/session/complete`](instance::Complete)
///     a. This increments the play count of the jig.
///
/// The hierarchy here is Jig -> Player Session -> Session Instance, where each arrow is a one-to-many mapping.
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
    type Res = JigPlayerSessionCreateResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/player";
    const METHOD: Method = Method::Post;
}

/// List the player session codes associated with a jig.
///
/// # Errors
///
/// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
/// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
pub struct List;
impl ApiEndpoint for List {
    type Req = ();
    type Res = JigPlayerSessionListResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/player";
    const METHOD: Method = Method::Get;
}

/// Endpoints for unauthed users to access jig player sessions.
pub mod instance {
    use crate::{
        api::{ApiEndpoint, Method},
        domain::jig::player::instance::{
            PlayerSessionInstanceCompleteRequest, PlayerSessionInstanceCreateRequest,
            PlayerSessionInstanceResponse,
        },
        error::EmptyError,
    };

    /// Create a session instance
    ///
    /// # Auth
    /// * No auth
    /// * Returns a token that needs to be cached. See #Flow section under [`player::Create`](super::Create)
    ///
    /// # Errors
    ///
    /// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
    /// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    /// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
    /// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig player session does not exist.
    pub struct Create;
    impl ApiEndpoint for Create {
        type Req = PlayerSessionInstanceCreateRequest;
        type Res = PlayerSessionInstanceResponse;
        type Err = EmptyError;
        const PATH: &'static str = "/v1/jig/player/instance";
        const METHOD: Method = Method::Post;
    }

    /// Complete a session instance and update the jig play count
    ///
    /// # Auth
    /// * Requires the token returned in [`Create`](Create)
    ///
    /// # Errors
    ///
    /// * [`400 - BadRequest`](http::StatusCode::BAD_REQUEST) if the request is malformed.
    /// * [`401 - Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
    /// * [`403 - Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
    /// * ['404 - NotFound'](http::StatusCode::NOT_FOUND) if the jig player session instance stored in the token does not exist.
    pub struct Complete;
    impl ApiEndpoint for Complete {
        type Req = PlayerSessionInstanceCompleteRequest;
        type Res = ();
        type Err = EmptyError;
        const PATH: &'static str = "/v1/jig/player/instance/complete";
        const METHOD: Method = Method::Post;
    }
}
