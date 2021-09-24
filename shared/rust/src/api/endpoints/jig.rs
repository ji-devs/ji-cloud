use crate::{
    api::Method,
    domain::{
        jig::{
            JigBrowseQuery, JigBrowseResponse, JigCountResponse, JigCreateRequest, JigId,
            JigResponse, JigSearchQuery, JigSearchResponse, JigUpdateDraftDataRequest,
            JigUpdateRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Endpoints for jig modules.
pub mod module;

/// Endpoints for jig additional resources.
pub mod additional_resource;

/// Endpoints for jig player sessions.
pub mod player;

/// Create a JIG.
///
/// * New jigs are all set to `PrivacyLevel::Unlisted` by default
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

/// Update a JIG. Note that this does not update the JIG's data.
///
pub struct Update;
impl ApiEndpoint for Update {
    type Req = JigUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Patch;
}

/// Get a JIG's live data by ID.
///
/// # Authorization
/// * None
///
/// # Errors
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent jig doesn't exist.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = JigResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/live";
    const METHOD: Method = Method::Get;
}

/// Get a JIG's draft data by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned JIGs
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent jig doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = JigResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Get;
}

/// Update the draft data of a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned JIGs
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = JigUpdateDraftDataRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/jig/{id}/draft";
    const METHOD: Method = Method::Patch;
}

/// Publish a JIG draft to live by copying over the JIG and module data.
///
/// # Authorization
/// * None
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/draft/publish";
    const METHOD: Method = Method::Put;
}

/// Browse JIGs. Returns the draft data copies in the response.
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

/// Clone a JIG. This clones both the draft and live.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the jig does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the JIG is a draft.
pub struct Clone;
impl ApiEndpoint for Clone {
    type Req = ();
    type Res = CreateResponse<JigId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/clone";
    const METHOD: Method = Method::Post;
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

/// Count of all public JIGs. See [`PrivacyLevel`](crate::domain::jig::PrivacyLevel).
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
