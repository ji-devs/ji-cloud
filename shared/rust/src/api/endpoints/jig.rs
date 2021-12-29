use crate::{
    api::Method,
    domain::{
        jig::{
            JigBrowseQuery, JigBrowseResponse, JigCountResponse, JigCreateRequest, JigId,
            JigLikedResponse, JigResponse, JigSearchQuery, JigSearchResponse,
            JigUpdateDraftDataRequest,
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

/// Create a JIG and it's draft and live data copies.
///
/// * New jigs are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a JIG and its two data copies with [`Create`]
/// 2. Optionally update JIG info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch Jig data through [`UpdateDraftData`]
///     b. Modify modules, through [`module::Update`]
/// 4. Finalize draft changes by calling [`Publish`]
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
/// Note that a copy of the JIG's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`JigData`](crate::domain::jig::JigData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned JIGs
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = JigUpdateDraftDataRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/jig/{id}";
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

/// Delete all jigs associated with current user.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned JIGs
pub struct DeleteAll;
impl ApiEndpoint for DeleteAll {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig";
    const METHOD: Method = Method::Delete;
}

/// Indicates that a jig has a cover
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned JIGs
pub struct Cover;
impl ApiEndpoint for Cover {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/cover";
    const METHOD: Method = Method::Patch;
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

/// Like a JIG
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Like;
impl ApiEndpoint for Like {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/like";
    const METHOD: Method = Method::Put;
}

/// Unlike a JIG
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Unlike;
impl ApiEndpoint for Unlike {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/like";
    const METHOD: Method = Method::Delete;
}

/// Is a JIG liked by a user
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Liked;
impl ApiEndpoint for Liked {
    type Req = ();
    type Res = JigLikedResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/like";
    const METHOD: Method = Method::Get;
}

/// Play a JIG
///
/// # Authorization
/// * None
pub struct Play;
impl ApiEndpoint for Play {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/play";
    const METHOD: Method = Method::Put;
}
