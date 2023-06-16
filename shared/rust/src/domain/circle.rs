//! Types for Circles.

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

use crate::api::endpoints::PathPart;
use strum_macros::Display;

use super::{asset::UserOrMe, image::ImageId, user::UserId};

wrap_uuid! {
    /// Wrapper type around [`Uuid`], represents the ID of a Circle.
    #[serde(rename_all = "camelCase")]
    pub struct CircleId
}

/// The response returned when a request for `GET`ing a Circle is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Circle {
    /// The ID of the Circle.
    pub id: CircleId,

    /// The Circle's name.
    pub display_name: String,

    /// Creator of Circle
    pub created_by: UserId,

    /// Description of the Circle. Defaults to empty string.
    pub description: String,

    /// Number of members on Circle
    pub member_count: u32,

    /// Image of Circle
    pub image: ImageId,

    /// When Circle was created
    pub created_at: DateTime<Utc>,

    /// When Circle was last edited
    pub last_edited: Option<DateTime<Utc>>,
}

make_path_parts!(CircleCreatePath => "/v1/circle");

/// Request to create a new Circle.
///
/// This creates the draft and live [Circle Data](Circle Data) copies with the requested info.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CircleCreateRequest {
    /// The Circle's name.
    pub display_name: String,

    /// Description of the Circle. Defaults to empty string.
    pub description: String,

    /// Image of the Circle
    pub image: ImageId,
}

make_path_parts!(UpdateCirclePath => "/v1/circle/{}" => CircleId);

/// Request for updating a Circle's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CircleUpdateRequest {
    /// The Circle's name to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// Description of the Circle to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// image of the Circle to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub image: Option<ImageId>,
}

make_path_parts!(CircleBrowsePath => "/v1/circle/browse");

/// Query for [`Browse`](crate::api::endpoints::circle::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CircleBrowseQuery {
    /// creator of circles
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserOrMe>,

    /// creator of circles
    /// The hits per page to be returned
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<UserId>,

    /// The page number to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,

    /// Order by sort
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
}

/// Response for [`Browse`](crate::api::endpoints::circle::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CircleBrowseResponse {
    /// the Circles returned.
    pub circles: Vec<Circle>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Circles found
    pub total_circle_count: u64,
}

make_path_parts!(CircleSearchPath => "/v1/circle");

/// Search for Circles via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CircleSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Circles to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by author's id
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserOrMe>,

    /// Optionally filter by the author's name
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Response for [`Search`](crate::api::endpoints::circle::Search).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CircleSearchResponse {
    /// the Circles returned.
    pub circles: Vec<Circle>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Circles found
    pub total_circle_count: u64,
}

make_path_parts!(CircleDeletePath => "/v1/circle/{}" => CircleId);

make_path_parts!(CircleGetPath => "/v1/circle/{}" => CircleId);

make_path_parts!(JoinCirclePath => "/v1/circle/{}/join" => CircleId);

make_path_parts!(LeaveCirclePath => "/v1/circle/{}/leave" => CircleId);

make_path_parts!(CircleRemoveMemberPath => "/v1/circle/{}/members/{}" => CircleId, UserId);

make_path_parts!(CircleBrowseMembersPath => "/v1/circle/{}/members" => CircleId);

/// Members associated with Circle
#[derive(Serialize, Deserialize)]
pub struct BrowseMembersResponse {
    /// user id of member
    pub members: Vec<UserId>,

    /// user id of member
    pub count: u32,
}

/// Sort browse results
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Display)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum OrderBy {
    /// Order Circles by member count
    #[strum(serialize = "MemberCount")]
    MemberCount = 0,
}
