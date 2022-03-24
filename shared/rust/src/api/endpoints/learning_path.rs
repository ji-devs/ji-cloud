use crate::{
    api::Method,
    domain::{
        learning_path::{
            LearningPathBrowseQuery, LearningPathBrowseResponse, LearningPathCreateRequest,
            LearningPathId, LearningPathResponse, LearningPathSearchQuery,
            LearningPathSearchResponse, LearningPathUpdateDraftDataRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Create a Learning Path and it's draft and live data copies.
///
/// * New Learning Paths are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a Learning Path and its two data copies with [`Create`]
/// 2. Optionally update Learning Path info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch Learning Path data through [`UpdateDraftData`]

/// 4. Finalize draft changes by calling [`Publish`]
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = LearningPathCreateRequest;
    type Res = CreateResponse<LearningPathId>;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/learning-path";
    const METHOD: Method = Method::Post;
}

/// Get a Learning Path's live data by ID.
///
/// # Authorization
/// * None
///
/// # Errors
///
pub struct GetLive;
impl ApiEndpoint for GetLive {
    type Req = ();
    type Res = LearningPathResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/learning-path/{id}/live";
    const METHOD: Method = Method::Get;
}

/// Get a Learning Path's draft data by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned Learning Paths
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
///
pub struct GetDraft;
impl ApiEndpoint for GetDraft {
    type Req = ();
    type Res = LearningPathResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/learning-path/{id}/draft";
    const METHOD: Method = Method::Get;
}

/// Update the draft data of a Learning Path.
///
/// Note that a copy of the Learning Path's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`Learning Path Data`](crate::domain::learning-path::LearningPathData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Learning Paths
pub struct UpdateDraftData;
impl ApiEndpoint for UpdateDraftData {
    type Req = LearningPathUpdateDraftDataRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/learning-path/{id}";
    const METHOD: Method = Method::Patch;
}

/// Publish a Learning Path draft to live by copying over the Learning Pathdata.
///
/// # Authorization
/// * None
pub struct Publish;
impl ApiEndpoint for Publish {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/learning-path/{id}/draft/publish";
    const METHOD: Method = Method::Put;
}

/// Browse Learning Paths. Returns the draft data copies in the response.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = LearningPathBrowseQuery;
    type Res = LearningPathBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/learning-path/browse";
    const METHOD: Method = Method::Get;
}

/// Search for Learning Paths.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = LearningPathSearchQuery;
    type Res = LearningPathSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/learning-path";
    const METHOD: Method = Method::Get;
}

/// Delete a Learning Path.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Learning Paths
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/learning-path/{id}";
    const METHOD: Method = Method::Delete;
}
