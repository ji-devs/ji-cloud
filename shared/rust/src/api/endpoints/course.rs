use crate::{
    api::Method,
    domain::{
        course::{
            CourseBrowseQuery, CourseBrowseResponse, CourseCreateRequest, CourseId, CourseResponse,
            CourseSearchQuery, CourseSearchResponse, CourseUpdateDraftDataRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

/// Endpoints for Course additional resources.
pub mod additional_resource;

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
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CourseCreateRequest;
    type Res = CreateResponse<CourseId>;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/course";
    const METHOD: Method = Method::Post;
}

/// Get a Course's live data by ID.
///
/// # Authorization
/// * None
///
/// # Errors
///
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = CourseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/course/{id}/live";
    const METHOD: Method = Method::Get;
}

/// Get a Course's draft data by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned Courses
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = CourseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/course/{id}/draft";
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
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/course/{id}";
    const METHOD: Method = Method::Patch;
}

/// Publish a Course draft to live by copying over the Coursedata.
///
/// # Authorization
/// * None
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/course/{id}/draft/publish";
    const METHOD: Method = Method::Put;
}

/// Browse Courses. Returns the draft data copies in the response.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = CourseBrowseQuery;
    type Res = CourseBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/course/browse";
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
    type Err = EmptyError;
    const PATH: &'static str = "/v1/course";
    const METHOD: Method = Method::Get;
}

/// Delete a Course.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/course/{id}";
    const METHOD: Method = Method::Delete;
}
