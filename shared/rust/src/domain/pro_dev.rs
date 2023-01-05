//! Types for ProDevs.
use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use self::unit::ProDevUnitId;

use super::{
    super::api::endpoints::PathPart,
    additional_resource::AdditionalResource,
    asset::{DraftOrLive, PrivacyLevel, UserOrMe},
    category::CategoryId,
    meta::ResourceTypeId,
    module::LiteModule,
    user::UserId,
};

pub mod unit;

/// Wrapper type around [`Uuid`], represents the ID of a ProDev.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, PathPart)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ProDevId(pub Uuid);

make_path_parts!(ProDevCreatePath => "/v1/pro-dev");

/// Request to create a new ProDev.
///
/// This creates the draft and live [ProDev Data](ProDev Data) copies with the requested info.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProDevCreateRequest {
    /// The ProDev's name.
    #[serde(default)]
    pub display_name: String,

    /// Description of the ProDev. Defaults to empty string.
    #[serde(default)]
    pub description: String,

    /// The language the ProDev uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(default)]
    pub language: String,

    /// The ProDev's categories.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub categories: Vec<CategoryId>,
}

/// The over-the-wire representation of a ProDev's data. This can either be the live copy or the draft copy.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProDevData {
    /// Whether the ProDev data is the live copy or the draft.
    pub draft_or_live: DraftOrLive,

    /// The ProDev's name.
    pub display_name: String,

    /// The language the ProDev uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    pub language: String,

    /// Description of the ProDev.
    pub description: String,

    /// When the ProDev was last edited
    pub last_edited: Option<DateTime<Utc>>,

    /// Duration of ProDev
    pub duration: Option<u32>,

    /// The privacy level on the ProDev.
    pub privacy_level: PrivacyLevel,

    /// Other keywords used to searched for ProDevs
    pub other_keywords: String,

    /// translated keywords used to searched for ProDevs
    pub translated_keywords: String,

    /// translated descriptions
    #[serde(default)]
    pub translated_description: HashMap<String, String>,

    /// This ProDev's cover.
    pub cover: Option<LiteModule>,

    /// The ProDev's categories.
    pub categories: Vec<CategoryId>,

    /// Additional resources of this ProDev.
    pub additional_resources: Vec<AdditionalResource>,

    /// List of ProDev Units within the ProDev
    pub units: Vec<ProDevUnitId>,
}

/// The response returned when a request for `GET`ing a ProDev is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProDevResponse {
    /// The ID of the ProDev.
    pub id: ProDevId,

    /// When (if at all) the ProDev has published a draft to live.
    pub published_at: Option<DateTime<Utc>>,

    /// The ID of the ProDev's original creator ([`None`] if unknown).
    pub creator_id: Option<UserId>,

    /// The current author
    pub author_id: Option<UserId>,

    /// The author's name, as "{given_name} {family_name}".
    pub author_name: Option<String>,

    /// Number of likes on ProDev
    pub likes: i64,

    /// Number of plays ProDev
    pub plays: i64,

    /// Live is current to Draft
    pub live_up_to_date: bool,

    /// The data of the requested ProDev.
    pub pro_dev_data: ProDevData,
}

make_path_parts!(ProDevGetLivePath => "/v1/pro-dev/{}/live" => ProDevId);

make_path_parts!(ProDevGetDraftPath => "/v1/pro-dev/{}/draft" => ProDevId);

make_path_parts!(ProDevUpdateDraftDataPath => "/v1/pro-dev/{}" => ProDevId);

/// Request for updating a ProDev's draft data.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProDevUpdateDraftDataRequest {
    /// The ProDev's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<UserId>,

    /// Description of the ProDev.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Estimated User Duration of the ProDev.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<u32>,

    /// The language the ProDev uses.
    ///
    /// NOTE: in the format `en`, `eng`, `en-US`, `eng-US` or `eng-USA`. To be replaced with a struct that enforces this.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub language: Option<String>,

    /// Privacy level for the ProDev.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_level: Option<PrivacyLevel>,

    /// Additional keywords for searches
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub other_keywords: Option<String>,

    /// The ProDev's categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,

    /// The ProDev's JIGs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub units: Option<Vec<ProDevUnitId>>,
}

make_path_parts!(ProDevPublishPath => "/v1/pro-dev/{}/draft/publish" => ProDevId);

make_path_parts!(ProDevBrowsePath => "/v1/pro-dev/browse");

/// Query for [`Browse`](crate::api::endpoints::pro_dev::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProDevBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// The page number of the ProDevs to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally browse by draft or live.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_or_live: Option<DraftOrLive>,

    /// Optionally filter ProDev by their privacy level
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

/// Response for [`Browse`](crate::api::endpoints::pro_dev::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProDevBrowseResponse {
    /// the ProDevs returned.
    pub pro_devs: Vec<ProDevResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of ProDevs found
    pub total_pro_dev_count: u64,
}

make_path_parts!(ProDevSearchPath => "/v1/pro-dev");

/// Search for ProDevs via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProDevSearchQuery {
    /// The query string.
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub q: String,

    /// The page number of the ProDevs to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `language`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Optionally filter by `is_published`. This means that the ProDev's `publish_at < now()`.
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

    /// Optionally search for ProDevs using keywords
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_keywords: Option<String>,

    /// Optionally search for ProDevs using translated keyword
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_keywords: Option<String>,

    /// Optionally search for ProDevs by privacy level
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

    /// Optionally filter by `categories`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<CategoryId>,

    /// Optionally filter by `units`
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(flatten)]
    pub units: Vec<ProDevUnitId>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProDevSearchResponse {
    /// the ProDevs returned.
    pub pro_devs: Vec<ProDevResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of ProDevs found
    pub total_pro_dev_count: u64,
}

make_path_parts!(ProDevDeletePath => "/v1/pro-dev/{}" => ProDevId);

into_uuid![ProDevId];
