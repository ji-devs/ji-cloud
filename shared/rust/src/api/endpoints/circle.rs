use crate::{
    api::Method,
    domain::{
        circle::{
            BrowseMembersResponse, Circle, CircleBrowseQuery, CircleBrowseResponse,
            CircleCreateRequest, CircleId, CircleSearchQuery, CircleSearchResponse,
            CircleUpdateRequest,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Create a Circle.

/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CircleCreateRequest;
    type Res = CreateResponse<CircleId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle";
    const METHOD: Method = Method::Post;
}

/// Update the Circle.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Update;
impl ApiEndpoint for Update {
    type Req = CircleUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/{id}";
    const METHOD: Method = Method::Patch;
}

/// Browse Circles. Returns the Circles in the response.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = CircleBrowseQuery;
    type Res = CircleBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/browse";
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
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle";
    const METHOD: Method = Method::Get;
}

/// Delete a Circle.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/{id}";
    const METHOD: Method = Method::Delete;
}

/// Get a Circle's by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned Circles
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the circle does not exist
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = Circle;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/{id}";
    const METHOD: Method = Method::Get;
}

/// Join a Circle.
pub struct JoinCircle;
impl ApiEndpoint for JoinCircle {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/{id}/join";
    const METHOD: Method = Method::Post;
}

/// Leave a Circle.
pub struct LeaveCircle;
impl ApiEndpoint for LeaveCircle {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/{id}/leave";
    const METHOD: Method = Method::Delete;
}

/// Browse members of a Circle.
pub struct BrowseMembers;
impl ApiEndpoint for BrowseMembers {
    type Req = ();
    type Res = BrowseMembersResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/circle/{id}/members";
    const METHOD: Method = Method::Get;
}
