use crate::{
    api::Method,
    domain::{
        badge::{
            Badge, BadgeBrowseQuery, BadgeBrowseResponse, BadgeCreateRequest, BadgeId,
            BadgeSearchQuery, BadgeSearchResponse, BadgeUpdateRequest, BrowseMembersResponse,
        },
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Create a Badge.

/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = BadgeCreateRequest;
    type Res = CreateResponse<BadgeId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge";
    const METHOD: Method = Method::Post;
}

/// Update the Badge.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Update;
impl ApiEndpoint for Update {
    type Req = BadgeUpdateRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/{id}";
    const METHOD: Method = Method::Patch;
}

/// Browse Badges. Returns the Badges in the response.
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = BadgeBrowseQuery;
    type Res = BadgeBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/browse";
    const METHOD: Method = Method::Get;
}

/// Search for Badges.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = BadgeSearchQuery;
    type Res = BadgeSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge";
    const METHOD: Method = Method::Get;
}

/// Delete a Badge.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/{id}";
    const METHOD: Method = Method::Delete;
}

/// Get a Badge's by ID.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`,, or `ManageSelfJig` for owned Badges
///
/// # Errors
/// * [`Unauthorized`](http::StatusCode::UNAUTHORIZED) if authorization is not valid.
/// * [`NotFound`](http::StatusCode::NOT_FOUND) if the badge does not exist
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = Badge;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/{id}";
    const METHOD: Method = Method::Get;
}

/// Join a Badge.
pub struct JoinBadge;
impl ApiEndpoint for JoinBadge {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/{id}";
    const METHOD: Method = Method::Post;
}

/// Leave a Badge.
pub struct LeaveBadge;
impl ApiEndpoint for LeaveBadge {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/{id}";
    const METHOD: Method = Method::Delete;
}

/// Browse members of a Badge.
pub struct BrowseMembers;
impl ApiEndpoint for BrowseMembers {
    type Req = ();
    type Res = BrowseMembersResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/badge/{id}/members";
    const METHOD: Method = Method::Get;
}
