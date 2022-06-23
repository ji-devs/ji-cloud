//! Types for Badges.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::asset::UserOrMe;

/// Wrapper type around [`Uuid`], represents the ID of a Badge.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct BadgeId(pub Uuid);

/// The response returned when a request for `GET`ing a Badge is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    /// The ID of the Badge.
    pub id: BadgeId,

    /// The Badge's name.
    pub display_name: String,

    /// Creator of Badge
    pub created_by: Uuid,

    /// Description of the Badge. Defaults to empty string.
    pub description: String,

    /// Number of members on Badge
    pub member_count: u32,

    /// Thumbnail URL
    pub thumbnail: url::Url,

    /// When Badge was created
    pub created_at: DateTime<Utc>,

    /// When Badge was last edited
    pub last_edited: Option<DateTime<Utc>>,
}

/// Request to create a new Badge.
///
/// This creates the draft and live [Badge Data](Badge Data) copies with the requested info.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BadgeCreateRequest {
    /// The Badge's name.
    pub display_name: String,

    /// Description of the Badge. Defaults to empty string.
    pub description: String,

    /// Thumbnail of the Badge
    pub thumbnail: url::Url,
}

/// Request for updating a Badge's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BadgeUpdateRequest {
    /// The Badge's name to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The current author to be updated
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creator_id: Option<Uuid>,

    /// Description of the Badge to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Thumbnail of the Badge to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub thumbnail: Option<url::Url>,
}

/// Query for [`Browse`](crate::api::endpoints::badge::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BadgeBrowseQuery {
    /// creator of badges
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

/// Response for [`Browse`](crate::api::endpoints::badge::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BadgeBrowseResponse {
    /// the Badges returned.
    pub badges: Vec<Badge>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Badges found
    pub total_badge_count: u64,
}

/// Search for Badges via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BadgeSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Badges to get.
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

/// Response for [`Search`](crate::api::endpoints::badge::Search).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BadgeSearchResponse {
    /// the Badges returned.
    pub badges: Vec<Badge>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Badges found
    pub total_badge_count: u64,
}

/// Members associated with Badge
#[derive(Serialize, Deserialize)]
pub struct BrowseMembersResponse {
    /// user id of member
    pub members: Vec<Uuid>,

    /// user id of member
    pub count: u32,
}

into_uuid![BadgeId];
