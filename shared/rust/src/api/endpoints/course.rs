use crate::{
    api::Method,
    domain::{
        course::{
            CourseBrowsePath, CourseBrowseQuery, CourseBrowseResponse, CourseClonePath,
            CourseCreatePath, CourseCreateRequest, CourseDeletePath, CourseGetDraftPath,
            CourseGetLivePath, CourseId, CoursePublishPath, CourseResponse, CourseSearchPath,
            CourseSearchQuery, CourseSearchResponse, CourseUpdateDraftDataPath,
            CourseUpdateDraftDataRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Create a Course and it's draft and live data copies.
///
/// * New Courses are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a Course and its two data copies with [`Create`]
/// 2. Optionally update Course info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch Course data through [`UpdateDraftData`]

/// 4. Finalize draft changes by calling [`Publish`]
///
/// # Authorization
/// * TokenUser
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CourseCreateRequest;
    type Res = CreateResponse<CourseId>;
    type Path = CourseCreatePath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Post;
}

/// Get a Course's live data by ID.
///
/// # Authorization
/// * Creator ID of Course
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned Courses
///
/// # Errors
///
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = CourseResponse;
    type Path = CourseGetLivePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Get a Course's draft data by ID.
///
/// # Authorization
/// * Creator ID of Course
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned Courses
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = CourseResponse;
    type Path = CourseGetDraftPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update the draft data of a Course.
///
/// Note that a copy of the Course's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`Course Data`](crate::domain::course::CourseData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = CourseUpdateDraftDataRequest;
    type Res = ();
    type Path = CourseUpdateDraftDataPath;
    type Err = MetadataNotFound;
    const METHOD: Method = Method::Patch;
}

/// Publish a Course draft to live by copying over the Coursedata.
///
/// # Authorization
/// * Creator ID of Course
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Path = CoursePublishPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Browse Courses. Returns the draft data copies in the response.
///
/// # Authorization
/// * None
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = CourseBrowseQuery;
    type Res = CourseBrowseResponse;
    type Path = CourseBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search for Courses.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = CourseSearchQuery;
    type Res = CourseSearchResponse;
    type Path = CourseSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Delete a Course.
///
/// # Authorization
/// * Creator ID of Course
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = CourseDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Clone a Course. This clones both the draft and live.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`Forbidden`](http::StatusCode::FORBIDDEN) if the user does not have sufficient permission to perform the action.
/// * ['NotFound'](http::StatusCode::NOT_FOUND) if the resource does not exist.
/// * ['BadRequest'](http::StatusCode::BAD_REQUEST) if the request is malformed or the Course is a draft.
pub struct Clone;
impl ApiEndpoint for Clone {
    type Path = CourseClonePath;
    type Req = ();
    type Res = CreateResponse<CourseId>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}
