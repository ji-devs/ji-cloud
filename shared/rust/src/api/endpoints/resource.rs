use crate::{
    api::Method,
    domain::{
        resource::{
            ResourceAdminDataUpdatePath, ResourceBrowsePath, ResourceBrowseQuery,
            ResourceBrowseResponse, ResourceClonePath, ResourceCountPath, ResourceCountResponse,
            ResourceCoverPath, ResourceCreatePath, ResourceCreateRequest, ResourceDeleteAllPath,
            ResourceDeletePath, ResourceGetDraftPath, ResourceGetLivePath, ResourceId,
            ResourceLikePath, ResourceLikedPath, ResourceLikedResponse, ResourcePlayPath,
            ResourcePublishPath, ResourceResponse, ResourceSearchPath, ResourceSearchQuery,
            ResourceSearchResponse, ResourceUnlikePath, ResourceUpdateAdminDataRequest,
            ResourceUpdateDraftDataPath, ResourceUpdateDraftDataRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Endpoints for resource curation.
pub mod curation;

/// Endpoints for resource reports.
pub mod report;

/// Create a Resource and it's draft and live data copies.
///
/// * New resources are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a Resource and its two data copies with [`Create`]
/// 2. Optionally update Resource info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch Resource data through [`UpdateDraftData`]
///     b. Modify modules, through [`module::Update`]
/// 4. Finalize draft changes by calling [`Publish`]
///
/// # Authorization
/// * One of `Admin`, `AdminResource`, or `ManageSelfResource`
pub struct Create;
impl ApiEndpoint for Create {
    type Path = ResourceCreatePath;
    type Req = ResourceCreateRequest;
    type Res = CreateResponse<ResourceId>;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Post;
}

/// Get a Resource's live data by ID.
///
/// # Authorization
/// * None
///
/// # Errors
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent resource doesn't exist.
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Path = ResourceGetLivePath;
    type Req = ();
    type Res = ResourceResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get a Resource's draft data by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminResource`,, or `ManageSelfResource` for owned Resources
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the module does not exist, or the parent resource doesn't exist.
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Path = ResourceGetDraftPath;
    type Req = ();
    type Res = ResourceResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update the draft data of a Resource.
///
/// Note that a copy of the Resource's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`ResourceData`](crate::domain::resource::ResourceData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminResource`,, or `ManageSelfResource` for owned Resources
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Path = ResourceUpdateDraftDataPath;
    type Req = ResourceUpdateDraftDataRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Patch;
}

/// Publish a Resource draft to live by copying over the Resource and module data.
///
/// # Authorization
/// * None
pub struct Publish;
impl ApiEndpoint for Publish {
    type Path = ResourcePublishPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Browse Resources. Returns the draft data copies in the response.
///
/// # Authorization
/// * One of `Admin`, `AdminResource`, or `ManageSelfResource`
pub struct Browse;
impl ApiEndpoint for Browse {
    type Path = ResourceBrowsePath;
    type Req = ResourceBrowseQuery;
    type Res = ResourceBrowseResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search for Resources.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Path = ResourceSearchPath;
    type Req = ResourceSearchQuery;
    type Res = ResourceSearchResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Clone a Resource. This clones both the draft and live.
///
/// # Authorization
/// * One of `Admin`, `AdminResource`, or `ManageSelfResource`
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the resource does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the Resource is a draft.
pub struct Clone;
impl ApiEndpoint for Clone {
    type Path = ResourceClonePath;
    type Req = ();
    type Res = CreateResponse<ResourceId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Delete a Resource.
///
/// # Authorization
/// * One of `Admin`, `AdminResource`, or `ManageSelfResource` for owned Resources
pub struct Delete;
impl ApiEndpoint for Delete {
    type Path = ResourceDeletePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Delete all resources associated with current user.
///
/// # Authorization
/// * One of `Admin`, `AdminResource`, or `ManageSelfResource` for owned Resources
pub struct DeleteAll;
impl ApiEndpoint for DeleteAll {
    type Path = ResourceDeleteAllPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Indicates that a resource has a cover
///
/// # Authorization
/// * One of `Admin`, `AdminResource`, or `ManageSelfResource` for owned Resources
pub struct Cover;
impl ApiEndpoint for Cover {
    type Path = ResourceCoverPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Count of all public Resources. See [`PrivacyLevel`](crate::domain::resource::PrivacyLevel).
///
/// # Authorization
/// * None
pub struct Count;
impl ApiEndpoint for Count {
    type Path = ResourceCountPath;
    type Req = ();
    type Res = ResourceCountResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Like a Resource
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Like;
impl ApiEndpoint for Like {
    type Path = ResourceLikePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Unlike a Resource
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Unlike;
impl ApiEndpoint for Unlike {
    type Path = ResourceUnlikePath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Is a Resource liked by a user
///
/// # Authorization
/// * Admin, BasicAuth
pub struct Liked;
impl ApiEndpoint for Liked {
    type Path = ResourceLikedPath;
    type Req = ();
    type Res = ResourceLikedResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Play a Resource
///
/// # Authorization
/// * None
pub struct Play;
impl ApiEndpoint for Play {
    type Path = ResourcePlayPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Update an admin data for a Resource.
///
/// # Authorization
///
/// * Standard + [`UserScope::ManageResource`](crate::domain::user::UserScope)
///
/// # Errors
///
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * [`BadRequest`](http::StatusCode::BAD_REQUEST) if the request is missing/invalid.
pub struct ResourceAdminDataUpdate;
impl ApiEndpoint for ResourceAdminDataUpdate {
    type Path = ResourceAdminDataUpdatePath;
    type Req = ResourceUpdateAdminDataRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}
