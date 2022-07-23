//! Types for Resources.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};
use uuid::Uuid;

pub mod curation;

pub mod report;
pub use report::{ReportId, ResourceReport};

use super::{
    additional_resource::AdditionalResource,
    asset::{DraftOrLive, OrderBy, PrivacyLevel, UserOrMe},
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, ResourceTypeId},
    module::LiteModule,
    user::UserId,
};

/// Wrapper type around [`Uuid`], represents the ID of a Resource.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ResourceId(pub Uuid);

/// The response returned when a request for `GET`ing a resource is successful.

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponse {
    /// The ID of the Resource.
    pub id: ResourceId,

    /// When (if at all) the Resource has published a draft to live.
    pub published_at: Option<DateTime<Utc>>,

    /// The ID of the Resource's original creator ([`None`] if unknown).
    pub creator_id: Option<UserId>,

    /// The current author
    pub author_id: Option<UserId>,

    /// The author's name, as "{given_name} {family_name}".
    pub author_name: Option<String>,

    /// Number of likes on Resource
    pub likes: i64,

    /// Number of plays Resource
    pub plays: i64,

    /// The data of the requested Resource.
    pub resource_data: ResourceData,

    /// Admin data for Resource
    pub admin_data: ResourceAdminData,
}

/// The over-the-wire representation of a Resource's data. This can either be the live copy or the draft copy.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceData {
    /// Whether the Resource data is the live copy or the draft.
    pub draft_or_live: DraftOrLive,

    /// The Resource's name.
    pub display_name: String,

    /// The Resource's remaining modules.
    ///
    /// NOTE: the first module will always exist and will always be of type cover
    pub cover: Option<LiteModule>,

    /// This resource's age ranges.
    pub age_ranges: Vec<AgeRangeId>,

    /// This resource's affiliations.
    pub affiliations: Vec<AffiliationId>,

    /// The language the resource uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: String,

    /// The resource's categories.
    pub categories: Vec<CategoryId>,

    /// Description of the resource.
    pub description: String,

    /// When the Resource was first created.
    pub created_at: DateTime<Utc>,

    /// When the resource was last edited
    pub last_edited: Option<DateTime<Utc>>,

    /// The privacy level on the Resource.
    pub privacy_level: PrivacyLevel,

    /// Lock this resource
    pub locked: bool,

    /// Other keywords used to searched for resources
    pub other_keywords: String,

    /// translated keywords used to searched for resources
    pub translated_keywords: String,

    /// translated descriptions
    #[serde(default)]
    pub translated_description: HashMap<String, String>,

    /// Additional resources of this Resource.
    pub additional_resources: Vec<AdditionalResource>,
}

/// Request to create a new Resource.
///
/// This creates the draft and live [ResourceData](ResourceData) copies with the requested info.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCreateRequest {
    /// The Resource's name.
    #[serde(default)]
    pub display_name: String,

    /// Description of the Resource. Defaults to empty string.
    #[serde(default)]
    pub description: String,

    /// This Resource's age ranges.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub age_ranges: Vec<AgeRangeId>,

    /// This Resource's affiliations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub affiliations: Vec<AffiliationId>,

    /// The language the Resource uses.
    ///
    /// If None, uses the user's language.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: Option<String>,

    /// The Resource's categories.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub categories: Vec<CategoryId>,
}

/// Request for updating a Resource's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUpdateDraftDataRequest {
    /// The Resource's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The language the Resource uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language: Option<String>,

    /// The Resource's categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,

    /// The Resource's age ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// The Resource's affiliations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affiliations: Option<Vec<AffiliationId>>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<UserId>,

    /// Description of the Resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Privacy level for the Resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_level: Option<PrivacyLevel>,

    /// Additional keywords for searches
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub other_keywords: Option<String>,
}

/// Query for [`Browse`](crate::api::endpoints::Resource::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// The page number of the Resources to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally browse by draft or live.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_or_live: Option<DraftOrLive>,

    /// Optionally filter Resource by their privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

    /// Optionally filter Resource by blocked status
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,

    /// Optionally filter by `additional resources`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceTypeId>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<OrderBy>,
}

/// Response for [`Browse`](crate::api::endpoints::Resource::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceBrowseResponse {
    /// the Resources returned.
    pub resources: Vec<ResourceResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Resources found
    pub total_resource_count: u64,
}

/// All id's associated with a Resource to delete
pub struct DeleteUserResources {
    /// Resource ID to delete.
    pub resource_id: ResourceId,
}

/// Search for Resources via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Resources to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `language`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Optionally filter by `age_ranges`
    ///
    /// Note: Currently does nothing
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<AgeRangeId>,

    /// Optionally filter by `affiliations`
    ///
    /// Note: Currently does nothing
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliations: Vec<AffiliationId>,

    /// Optionally filter by `additional resources`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceTypeId>,

    /// Optionally filter by `categories`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<CategoryId>,

    /// Optionally filter by `is_published`. This means that the Resource's `publish_at < now()`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author's id
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// Optionally filter by the author's name
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,

    /// Optionally search for Resources using keywords
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_keywords: Option<String>,

    /// Optionally search for Resources using translated keyword
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_keywords: Option<String>,

    /// Optionally search for Resources by privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

    /// Optionally search for blocked or non-blocked Resources
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSearchResponse {
    /// the resources returned.
    pub resources: Vec<ResourceResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of resources found
    pub total_resource_count: u64,
}

/// Response for successfully finding the draft of a resource.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceIdResponse {
    /// The ID of the resource
    pub id: ResourceId,
}

/// Response for total count of public and published resource.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCountResponse {
    /// Total number of public and published resources.
    pub total_count: u64,
}

/// Response for whether a user has liked a Resource.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceLikedResponse {
    /// Whether the authenticated user has liked the current Resource
    pub is_liked: bool,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAdminData {
    /// Rating for resource, weighted for resource search
    #[serde(default)]
    pub rating: Option<ResourceRating>,

    /// if true does not appear in search
    pub blocked: bool,

    /// Indicates resource has been curated by admin
    pub curated: bool,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUpdateAdminDataRequest {
    /// Rating for resource, weighted for resource search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<ResourceRating>,

    /// if true does not appear in search
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked: Option<bool>,

    /// Indicates resource has been curated by admin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub curated: Option<bool>,
}

/// These fields can be edited by admin and can be viewed by everyone
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAdminUpdateData {
    /// Rating for resource, weighted for resource search
    pub rating: Option<Option<ResourceRating>>,

    /// if true does not appear in search
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blocked: Option<bool>,

    /// Indicates resource has been curated by admin
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub curated: Option<bool>,
}

/// Admin rating for Resource
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum ResourceRating {
    #[allow(missing_docs)]
    One = 1,
    #[allow(missing_docs)]
    Two = 2,
    #[allow(missing_docs)]
    Three = 3,
}

impl TryFrom<u8> for ResourceRating {
    type Error = ();

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        match num {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

into_uuid![ResourceId];
