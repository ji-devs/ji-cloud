use crate::{
    api::Method,
    domain::{
        circle::{
            BrowseMembersResponse, Circle, CircleBrowseMembersPath, CircleBrowsePath,
            CircleBrowseQuery, CircleBrowseResponse, CircleCreatePath, CircleCreateRequest,
            CircleDeletePath, CircleGetPath, CircleId, CircleRemoveMemberPath, CircleSearchPath,
            CircleSearchQuery, CircleSearchResponse, CircleUpdateRequest, JoinCirclePath,
            LeaveCirclePath, UpdateCirclePath,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Create a Circle.
///
/// # Authorization
/// * TokenUser
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CircleCreateRequest;
    type Res = CreateResponse<CircleId>;
    type Path = CircleCreatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Update the Circle.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned Circles
///
pub struct Update;
impl ApiEndpoint for Update {
    type Req = CircleUpdateRequest;
    type Res = ();
    type Path = UpdateCirclePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Browse Circles. Returns the Circles in the response.
///
/// # Authorization
/// * TokenUser of owned Circles
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned Circles
///
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = CircleBrowseQuery;
    type Res = CircleBrowseResponse;
    type Path = CircleBrowsePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Search for Circles.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = CircleSearchQuery;
    type Res = CircleSearchResponse;
    type Path = CircleSearchPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Delete a Circle.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`, or `ManageSelfAsset` for owned Circles
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = CircleDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Get a Circle's by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminAsset`,, or `ManageSelfAsset` for owned Circles
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the circle does not exist
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = Circle;
    type Path = CircleGetPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Join a Circle.
/// # Authorization
/// * TokenUser
/// * One of `Admin`, `AdminAsset`,, or `ManageSelfAsset` for owned Circles
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the circle does not exist
pub struct JoinCircle;
impl ApiEndpoint for JoinCircle {
    type Req = ();
    type Res = ();
    type Path = JoinCirclePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Leave a Circle.
pub struct LeaveCircle;
impl ApiEndpoint for LeaveCircle {
    type Req = ();
    type Res = ();
    type Path = LeaveCirclePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Remove member from a Circle.
pub struct RemoveMember;
impl ApiEndpoint for RemoveMember {
    type Req = ();
    type Res = ();
    type Path = CircleRemoveMemberPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Browse members of a Circle.
pub struct BrowseMembers;
impl ApiEndpoint for BrowseMembers {
    type Req = ();
    type Res = BrowseMembersResponse;
    type Path = CircleBrowseMembersPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
