//! Types for Circles.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{asset::UserOrMe, image::ImageId, user::UserId};

/// Wrapper type around [`Uuid`], represents the ID of a Circle.
#[derive(Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct CircleId(pub Uuid);

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
    pub image: Option<ImageId>,

    /// When Circle was created
    pub created_at: DateTime<Utc>,

    /// When Circle was last edited
    pub last_edited: Option<DateTime<Utc>>,
}

/// Request to create a new Circle.
///
/// This creates the draft and live [Circle Data](Circle Data) copies with the requested info.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CircleCreateRequest {
    /// The Circle's name.
    pub display_name: String,

    /// Description of the Circle. Defaults to empty string.
    pub description: String,

    /// Image of the Circle
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub image: Option<ImageId>,
}

/// Request for updating a Circle's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CircleUpdateRequest {
    /// The Circle's name to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The current author to be updated
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creator_id: Option<UserId>,

    /// Description of the Circle to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// image of the Circle to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub image: Option<ImageId>,
}

/// Query for [`Browse`](crate::api::endpoints::circle::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CircleBrowseQuery {
    /// creator of circles
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserOrMe>,

    /// The page number to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
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

/// Members associated with Circle
#[derive(Serialize, Deserialize)]
pub struct BrowseMembersResponse {
    /// user id of member
    pub members: Vec<UserId>,

    /// user id of member
    pub count: u32,
}

into_uuid![CircleId];
