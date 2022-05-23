use crate::{
    api::Method,
    domain::{
        badge::{
            Badge, BadgeBrowseQuery, BadgeBrowseResponse, BadgeCreateRequest, BadgeId,
            BadgeUpdateRequest, BrowseMembersResponse,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Create a Badge and it's draft and live data copies.
///
/// * New Courses are all set to `PrivacyLevel::Unlisted` by default
///
/// # Flow:
/// 1. Create a Badge and its two data copies with [`Create`]
/// 2. Optionally update Badge info such as privacy, author with [`Update`]
/// 3. Make updates to draft data:
///     a. Patch Badge data through [`UpdateDraftData`]

/// 4. Finalize draft changes by calling [`Publish`]
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = BadgeCreateRequest;
    type Res = CreateResponse<BadgeId>;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/badge";
    const METHOD: Method = Method::Post;
}

/// Update the draft data of a Badge.
///
/// Note that a copy of the Badge's draft or live data can not be fetched directly, but only as a part
/// of one of the following routes:
/// * [`GetLive`] fetches live copies
/// * [`Search`]
///
/// See [`Badge Data`](crate::domain::course::CourseData) for the over-the-wire representation.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Courses
pub struct Update;
impl ApiEndpoint for Update {
    type Req = BadgeUpdateRequest;
    type Res = ();
    type Err = MetadataNotFound;
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

// /// Search for Courses.
// ///
// /// # Authorization
// /// * None
// pub struct Search;
// impl ApiEndpoint for Search {
//     type Req = BadgeSearchQuery;
//     type Res = BadgeSearchResponse;
//     type Err = EmptyError;
//     const PATH: &'static str = "/v1/badge";
//     const METHOD: Method = Method::Get;
// }

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

/// Get a Badge's draft data by ID.
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
    const PATH: &'static str = "/v1/badge/{id}";
    const METHOD: Method = Method::Get;
}
