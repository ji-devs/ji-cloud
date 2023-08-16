//! Types for Resource short codes for sharing
use crate::{DateTime, Utc};
use macros::make_path_parts;
use mymacros::{Deserialize, Serialize};

use crate::api::endpoints::PathPart;

use super::{report::ResourceReport, ResourceId, UserId};

wrap_uuid! {
    /// Wrapper type around [`Uuid`](Uuid), represents the ID of a curation comment.
    pub struct CommentId
}

make_path_parts!(ResourceCurationPath => "/v1/resource/{}/curation" => ResourceId);

/// Curation data for Resources
#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct ResourceCurationData {
    /// Resource ID for curation
    pub resource_id: ResourceId,

    /// Fields curated by Admin
    pub fields_done: ResourceCurationFieldsDone,

    /// Status for curation
    pub curation_status: ResourceCurationStatus,

    /// Comments from curator (not updatable)
    pub comments: Vec<ResourceCurationComment>,

    /// Reports for Resource from users (not updatable)
    pub reports: Vec<ResourceReport>,
}

/// Curation fields that have been completed
///
/// Authorization:
/// Admin
#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct ResourceCurationFieldsDone {
    /// Display name of Resource
    pub display_name: bool,

    /// Language of Resource
    pub language: bool,

    /// Categories of Resource
    pub categories: bool,

    /// Descriptions of Resource
    pub description: bool,

    /// Age ranges of Resource
    pub age_ranges: bool,

    /// Affiliations of Resource
    pub affiliations: bool,

    /// Addtional resources of Resource
    pub additional_resources: bool,
}

impl Default for ResourceCurationFieldsDone {
    fn default() -> Self {
        Self {
            display_name: false,
            language: false,
            categories: false,
            description: false,
            age_ranges: false,
            affiliations: false,
            additional_resources: false,
        }
    }
}

/// Status of Curation
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
// #[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum ResourceCurationStatus {
    /// Resources first curation
    New = 0,

    /// Resource has been updated by user
    NewVersion = 1,

    /// Admin is reviewing Resource
    InProgress = 2,

    /// Curation of Resource completed
    Done = 3,
}

impl Default for ResourceCurationStatus {
    fn default() -> Self {
        Self::New
    }
}

make_path_parts!(ResourceCurationUpdatePath => "/v1/resource/{}/curation" => ResourceId);

/// Curation data for ResourceS
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct ResourceCurationUpdateRequest {
    /// Display name of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<bool>,

    /// Language of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<bool>,

    /// Categories of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<bool>,

    /// Descriptions of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<bool>,

    /// Age ranges of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub age_ranges: Option<bool>,

    /// Affiliations of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliations: Option<bool>,

    /// Addtional resources of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_resources: Option<bool>,

    /// Curation status of Resource
    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub curation_status: Option<ResourceCurationStatus>,
}

make_path_parts!(ResourceCurationCommentCreatePath => "/v1/resource/{}/curation/comment" => ResourceId);

/// Curation data for ResourceS
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
// #[serde(rename_all = "camelCase")]
pub struct ResourceCurationComment {
    /// Comment ID
    pub id: CommentId,

    /// Resource ID for comment
    pub resource_id: ResourceId,

    /// Comment
    pub value: String,

    /// When comment was submitted
    pub created_at: DateTime<Utc>,

    /// ID of commenter
    pub author_id: UserId,
}

/// Request to comment on Resource
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct ResourceCurationCommentRequest {
    /// Display name of Resource
    pub value: String,
}

make_path_parts!(ResourceCurationCommentGetPath => "/v1/resource/{}/curation/comment/{}" => ResourceId, CommentId);

/// Curation data for ResourceS
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct ResourceCurationCommentResponse {
    /// ID of comment
    pub id: CommentId,

    /// ID of Resource
    pub resource_id: ResourceId,

    /// Curator comment
    pub value: String,

    /// When comment was submitted
    pub created_at: Option<DateTime<Utc>>,

    /// ID of commenter
    pub author_id: UserId,

    /// Name of commenter
    pub author_name: String,
}
