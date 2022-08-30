//! Types for Courses.

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    super::api::endpoints::PathPart,
    additional_resource::AdditionalResource,
    asset::{DraftOrLive, PrivacyLevel, UserOrMe},
    category::CategoryId,
    jig::JigId,
    meta::{AffiliationId, AgeRangeId, ResourceTypeId},
    module::LiteModule,
    user::UserId,
};

/// Wrapper type around [`Uuid`], represents the ID of a Course.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, PathPart)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct CourseId(pub Uuid);

make_path_parts!(CourseCreatePath => "/v1/course");

/// Request to create a new Course.
///
/// This creates the draft and live [Course Data](Course Data) copies with the requested info.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CourseCreateRequest {
    /// The Course's name.
    #[serde(default)]
    pub display_name: String,

    /// Description of the Course. Defaults to empty string.
    #[serde(default)]
    pub description: String,

    /// This Course's age ranges.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub age_ranges: Vec<AgeRangeId>,

    /// This Course's affiliations.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub affiliations: Vec<AffiliationId>,

    /// The language the Course uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(default)]
    pub language: String,

    /// The Course's categories.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub categories: Vec<CategoryId>,
}

/// The over-the-wire representation of a Course's data. This can either be the live copy or the draft copy.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CourseData {
    /// Whether the Course data is the live copy or the draft.
    pub draft_or_live: DraftOrLive,

    /// The Course's name.
    pub display_name: String,

    /// The language the Course uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: String,

    /// Description of the Course.
    pub description: String,

    /// When the Course was last edited
    pub last_edited: Option<DateTime<Utc>>,

    /// The privacy level on the Course.
    pub privacy_level: PrivacyLevel,

    /// Other keywords used to searched for Courses
    pub other_keywords: String,

    /// translated keywords used to searched for Courses
    pub translated_keywords: String,

    /// translated descriptions
    #[serde(default)]
    pub translated_description: HashMap<String, String>,

    /// This Course's cover.
    pub cover: Option<LiteModule>,

    /// This Course's age ranges.
    pub age_ranges: Vec<AgeRangeId>,

    /// This Course's affiliations.
    pub affiliations: Vec<AffiliationId>,

    /// The Course's categories.
    pub categories: Vec<CategoryId>,

    /// Additional resources of this Course.
    pub additional_resources: Vec<AdditionalResource>,

    /// List of Jig Ids within the Course
    pub items: Vec<JigId>,
}

/// The response returned when a request for `GET`ing a Course is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CourseResponse {
    /// The ID of the Course.
    pub id: CourseId,

    /// When (if at all) the Course has published a draft to live.
    pub published_at: Option<DateTime<Utc>>,

    /// The ID of the Course's original creator ([`None`] if unknown).
    pub creator_id: Option<UserId>,

    /// The current author
    pub author_id: Option<UserId>,

    /// The author's name, as "{given_name} {family_name}".
    pub author_name: Option<String>,

    /// Number of likes on Course
    pub likes: i64,

    /// Number of plays Course
    pub plays: i64,

    /// The data of the requested Course.
    pub course_data: CourseData,
}

make_path_parts!(CourseGetLivePath => "/v1/course/{}/live" => CourseId);

make_path_parts!(CourseGetDraftPath => "/v1/course/{}/draft" => CourseId);

make_path_parts!(CourseUpdateDraftDataPath => "/v1/course/{}" => CourseId);

/// Request for updating a Course's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CourseUpdateDraftDataRequest {
    /// The Course's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<UserId>,

    /// Description of the Course.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// The language the Course uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language: Option<String>,

    /// Privacy level for the Course.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_level: Option<PrivacyLevel>,

    /// Additional keywords for searches
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub other_keywords: Option<String>,

    /// The Course's categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,

    /// The Course's age ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// The Course's affiliations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affiliations: Option<Vec<AffiliationId>>,

    /// The Course's JIGs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub items: Option<Vec<JigId>>,
}

make_path_parts!(CoursePublishPath => "/v1/course/{}/draft/publish" => CourseId);

make_path_parts!(CourseBrowsePath => "/v1/course/browse");

/// Query for [`Browse`](crate::api::endpoints::course::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CourseBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// The page number of the Courses to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally browse by draft or live.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_or_live: Option<DraftOrLive>,

    /// Optionally filter Course by their privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

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
}

/// Response for [`Browse`](crate::api::endpoints::course::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CourseBrowseResponse {
    /// the Courses returned.
    pub courses: Vec<CourseResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Courses found
    pub total_course_count: u64,
}

make_path_parts!(CourseSearchPath => "/v1/course");

/// Search for Courses via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CourseSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the Courses to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `language`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Optionally filter by `is_published`. This means that the Course's `publish_at < now()`.
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

    /// Optionally search for Courses using keywords
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_keywords: Option<String>,

    /// Optionally search for Courses using translated keyword
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_keywords: Option<String>,

    /// Optionally search for Courses by privacy level
    #[serde(default)]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub privacy_level: Vec<PrivacyLevel>,

    /// The hits per page to be returned
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_limit: Option<u32>,

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

    /// Optionally filter by `items`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<JigId>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CourseSearchResponse {
    /// the Courses returned.
    pub courses: Vec<CourseResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of Courses found
    pub total_course_count: u64,
}

make_path_parts!(CourseDeletePath => "/v1/course/{}" => CourseId);

into_uuid![CourseId];
