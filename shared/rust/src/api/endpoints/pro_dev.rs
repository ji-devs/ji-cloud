//! Endpoints for ProDev
use crate::{
    api::Method,
    domain::{
        pro_dev::{
            ProDevBrowsePath, ProDevBrowseQuery, ProDevBrowseResponse, ProDevClonePath,
            ProDevCreatePath, ProDevCreateRequest, ProDevDeletePath, ProDevGetDraftPath,
            ProDevGetLivePath, ProDevId, ProDevPublishPath, ProDevResponse, ProDevSearchPath,
            ProDevSearchQuery, ProDevSearchResponse, ProDevUpdateDraftDataPath,
            ProDevUpdateDraftDataRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

pub mod unit;

use super::ApiEndpoint;

/// Create a ProDev and it's draft and live data copies.
///
/// * New ProDevs are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a ProDev and its two data copies with [`Create`]
/// 2. Optionally update ProDev info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch ProDev data through [`UpdateDraftData`]

/// 4. Finalize draft changes by calling [`Publish`]
///
/// # Authorization
/// * TokenUser
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = ProDevCreateRequest;
    type Res = CreateResponse<ProDevId>;
    type Path = ProDevCreatePath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Post;
}

/// Get a ProDev's live data by ID.
///
/// # Authorization
/// * Creator ID of ProDev
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned ProDevs
///
/// # Errors
///
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = ProDevResponse;
    type Path = ProDevGetLivePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get a ProDev's draft data by ID.
///
/// # Authorization
/// * Creator ID of ProDev
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned ProDevs
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = ProDevResponse;
    type Path = ProDevGetDraftPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update the draft data of a ProDev.
///
/// Note that a copy of the ProDev's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`ProDev Data`](crate::domain::pro_dev::ProDevData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned ProDevs
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = ProDevUpdateDraftDataRequest;
    type Res = ();
    type Path = ProDevUpdateDraftDataPath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Patch;
}

/// Publish a ProDev draft to live by copying over the ProDevdata.
///
/// # Authorization
/// * Creator ID of ProDev
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Path = ProDevPublishPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Browse ProDevs. Returns the draft data copies in the response.
///
/// # Authorization
/// * None
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = ProDevBrowseQuery;
    type Res = ProDevBrowseResponse;
    type Path = ProDevBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search for ProDevs.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = ProDevSearchQuery;
    type Res = ProDevSearchResponse;
    type Path = ProDevSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Delete a ProDev.
///
/// # Authorization
/// * Creator ID of ProDev
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned ProDevs
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = ProDevDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Clone a ProDev. This clones both the draft and live.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the resource does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the ProDev is a draft.
pub struct Clone;
impl ApiEndpoint for Clone {
    type Path = ProDevClonePath;
    type Req = ();
    type Res = CreateResponse<ProDevId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}
