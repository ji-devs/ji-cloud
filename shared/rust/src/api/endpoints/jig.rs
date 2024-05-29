use crate::{
    api::Method,
    domain::{
        jig::{
            GetJigPlaylistsPath, GetJigPlaylistsResponse, JigAdminDataUpdatePath,
            JigAdminTransferRequest, JigBrowsePath, JigBrowseQuery, JigBrowseResponse,
            JigClonePath, JigCountPath, JigCountResponse, JigCoverPath, JigCreatePath,
            JigCreateRequest, JigDeleteAllPath, JigDeletePath, JigFeaturedPath,
            JigFeaturedResponse, JigFeaturedUpdateRequest, JigGetDraftPath, JigGetLivePath, JigId,
            JigLikePath, JigLikedPath, JigLikedResponse, JigPlayPath, JigPublishPath, JigResponse,
            JigSearchPath, JigSearchQuery, JigSearchResponse, JigTransferAdminPath,
            JigTrendingPath, JigTrendingResponse, JigUnlikePath, JigUpdateAdminDataRequest,
            JigUpdateDraftDataPath, JigUpdateDraftDataRequest, ListLikedPath, ListLikedRequest,
            ListLikedResponse,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Endpoints for jig codes.
pub mod codes;

/// Endpoints for jig curation.
pub mod curation;

/// Endpoints for jig reports.
pub mod report;

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
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = JigCreateRequest;
    type Res = CreateResponse<JigId>;
    type Path = JigCreatePath;
    type Err = MetadataNotFound;
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
    type Path = JigGetLivePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get a JIG's draft data by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`,, or `ManageSelfAsset` for owned JIGs
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent jig doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = JigResponse;
    type Path = JigGetDraftPath;
    type Err = EmptyError;
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
/// * One of `Admin`, `AdminAsset`,, or `ManageSelfAsset` for owned JIGs
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = JigUpdateDraftDataRequest;
    type Res = ();
    type Path = JigUpdateDraftDataPath;
    type Err = MetadataNotFound;
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
    type Path = JigPublishPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Browse JIGs. Returns the draft data copies in the response.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
// TODO: Think there should be a BrowseLive and a BrowseDraft endpoint.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = JigBrowseQuery;
    type Res = JigBrowseResponse;
    type Path = JigBrowsePath;
    type Err = EmptyError;
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
    type Path = JigSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Trending JIGs.
pub struct Trending;
impl ApiEndpoint for Trending {
    type Req = ();
    type Res = JigTrendingResponse;
    type Path = JigTrendingPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// List user's liked JIGs.
pub struct ListLiked;
impl ApiEndpoint for ListLiked {
    type Req = ListLikedRequest;
    type Res = ListLikedResponse;
    type Path = ListLikedPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Featured JIGs.
pub struct Featured;
impl ApiEndpoint for Featured {
    type Req = ();
    type Res = JigFeaturedResponse;
    type Path = JigFeaturedPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update featured JIGs.
pub struct FeaturedUpdate;
impl ApiEndpoint for FeaturedUpdate {
    type Req = JigFeaturedUpdateRequest;
    type Res = ();
    type Path = JigFeaturedPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Clone a JIG. This clones both the draft and live.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
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
    type Path = JigClonePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Delete a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned JIGs
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = JigDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Delete all jigs associated with current user.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned JIGs
pub struct DeleteAll;
impl ApiEndpoint for DeleteAll {
    type Req = ();
    type Res = ();
    type Path = JigDeleteAllPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Indicates that a jig has a cover
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned JIGs
pub struct Cover;
impl ApiEndpoint for Cover {
    type Req = ();
    type Res = ();
    type Path = JigCoverPath;
    type Err = EmptyError;
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
    type Path = JigCountPath;
    type Err = EmptyError;
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
    type Path = JigLikePath;
    type Err = EmptyError;
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
    type Path = JigUnlikePath;
    type Err = EmptyError;
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
    type Path = JigLikedPath;
    type Err = EmptyError;
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
    type Path = JigPlayPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Update an admin data for a JIG.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct JigAdminDataUpdate;
impl ApiEndpoint for JigAdminDataUpdate {
    type Req = JigUpdateAdminDataRequest;
    type Res = ();
    type Path = JigAdminDataUpdatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Update an admin data for a JIG.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct JigAdminTransfer;
impl ApiEndpoint for JigAdminTransfer {
    type Req = JigAdminTransferRequest;
    type Res = ();
    type Path = JigTransferAdminPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Remove resource from jigs algolia
///
/// # NOTE
/// * remove after resources are separated
///
/// # Authorization
/// * Admin
pub struct RemoveResource;
impl ApiEndpoint for RemoveResource {
    type Path = RemoveResourcePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
use crate::api::endpoints::PathPart;
macros::make_path_parts!(RemoveResourcePath => "/v1/jig/{}/resources" => JigId);

/// Playlists containing JIG id
///
/// # Authorization
/// * Standard + [`UserScope::ManageJig`](crate::domain::user::UserScope)
///
pub struct GetJigPlaylists;
impl ApiEndpoint for GetJigPlaylists {
    type Req = ();
    type Res = GetJigPlaylistsResponse;
    type Path = GetJigPlaylistsPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
