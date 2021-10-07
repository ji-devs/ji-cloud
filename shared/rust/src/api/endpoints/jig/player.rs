use crate::{
    api::{ApiEndpoint, Method},
    domain::jig::player::{
        JigPlayCountResponse, JigPlayerSessionCreateRequest, JigPlayerSessionCreateResponse,
        JigPlayerSessionListResponse,
    },
    error::EmptyError,
};

/// Create a player session from a jig. Requestor needs permissions over the jig.
///
/// # Flow
///
/// 1. Author/admin creates a player session using [`POST /v1/jig/player`](Create)
///     * This is represented by a *session code/index*
/// 2. Unauthed user instantiates the player session. This creates an instance of a session. [`POST /v1/jig/player/session`](instance::Create) returns:
///     * A short lived token, which identifies the guest user and the session instance.
///     * The player session settings.
///     * `JigId` of the JIG on which the session was created.
/// 3. Unauthed user posts short lived token to complete the instance. [`POST /v1/jig/player/session/complete`](instance::Complete)
///     * This increments the play count of the jig.
///     * Deletes the completed instance from the DB.
///
/// The hierarchy here is Jig -> Player Session -> Session Instance, where each arrow is a one-to-many mapping.
///
/// Each level is uniquely identified by `JigId` -> `JigPlayerSessionIndex` -> token.
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
/// 1. Generate a random code in the range 0..MAX
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

/// Number of times a JIG has been played.
///
/// # Authorization
/// * None
pub struct PlayCount;
impl ApiEndpoint for PlayCount {
    type Req = ();
    type Res = JigPlayCountResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/play-count";
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
