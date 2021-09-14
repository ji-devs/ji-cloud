use crate::{
    api::Method,
    domain::{
        jig::{
            JigBrowseQuery, JigBrowseResponse, JigCountResponse, JigCreateRequest, JigId,
            JigResponse, JigSearchQuery, JigSearchResponse, JigUpdateRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;
use crate::domain::jig::player::JigPlayCountResponse;

/// Endpoints for jig modules.
pub mod module;

/// Endpoints for jig additional resources.
pub mod additional_resource;

/// Endpoints for jig drafts.
pub mod draft;

/// Endpoints for jig player sessions.
pub mod player;

/// Get a JIG by ID.
///
/// # Authorization
/// * None
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = JigResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Get;
}

/// Browse JIGs.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = JigBrowseQuery;
    type Res = JigBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/browse";
    const METHOD: Method = Method::Get;
}

/// Search for JIGs.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = JigSearchQuery;
    type Res = JigSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig";
    const METHOD: Method = Method::Get;
}

/// Create a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = JigCreateRequest;
    type Res = CreateResponse<JigId>;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/jig";
    const METHOD: Method = Method::Post;
}

/// Clone a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
///
/// # Errors
/// [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
/// [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
///
/// ['NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
///
/// ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the JIG is a draft.
pub struct Clone;
impl ApiEndpoint for Clone {
    type Req = ();
    type Res = CreateResponse<JigId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/clone";
    const METHOD: Method = Method::Post;
}

/// Update a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned JIGs
pub struct Update;
impl ApiEndpoint for Update {
    type Req = JigUpdateRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned JIGs
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Delete;
}

/// Count of public and published JIGs.
///
/// # Authorization
/// * None
pub struct Count;
impl ApiEndpoint for Count {
    type Req = ();
    type Res = JigCountResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/count";
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
