//! Types for Jig short codes for sharing
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{report::JigReport, JigId};

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a curation comment.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct CommentId(pub Uuid);

/// Curation data for JIGS
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigCurationData {
    /// Jig ID for curation
    pub jig_id: JigId,

    /// Fields curated by Admin
    pub fields_done: JigCurationFieldsDone,

    /// Status for curation
    pub curation_status: JigCurationStatus,

    /// Comments from curator (not updatable)
    pub comments: Vec<JigCurationComment>,

    /// Reports for Jig from users (not updatable)
    pub reports: Vec<JigReport>,
}

/// Curation fields that have been completed
///
/// Authorization:
/// Admin
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigCurationFieldsDone {
    /// Display name of JIG
    pub display_name: bool,

    /// Language of JIG
    pub language: bool,

    /// Categories of JIG
    pub categories: bool,

    /// Descriptions of JIG
    pub description: bool,

    /// Age ranges of JIG
    pub age_ranges: bool,

    /// Affiliations of JIG
    pub affiliations: bool,

    /// Addtional resources of JIG
    pub additional_resources: bool,
}

impl Default for JigCurationFieldsDone {
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
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum JigCurationStatus {
    /// JIGs first curation
    New = 0,

    /// JIG has been updated by user
    NewVersion = 1,

    /// Admin is reviewing JIG
    InProgress = 2,

    /// Curation of JIG completed
    Done = 3,
}

impl Default for JigCurationStatus {
    fn default() -> Self {
        Self::New
    }
}

/// Curation data for JIGS
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigCurationUpdateRequest {
    /// Display name of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<bool>,

    /// Language of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<bool>,

    /// Goals of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<bool>,

    /// Categories of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<bool>,

    /// Descriptions of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<bool>,

    /// Age ranges of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_ranges: Option<bool>,

    /// Affiliations of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliations: Option<bool>,

    /// Addtional resources of JIG
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_resources: Option<bool>,

    /// Curation status of Jig
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curation_status: Option<JigCurationStatus>,
}

/// Curation data for JIGS
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigCurationComment {
    /// Comment ID
    pub id: CommentId,

    /// Jig ID for comment
    pub jig_id: JigId,

    /// Comment
    pub value: String,

    /// When comment was submitted
    pub created_at: DateTime<Utc>,

    /// ID of commenter
    pub author_id: Uuid,
}

/// Request to comment on Jig
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigCurationCommentRequest {
    /// Display name of JIG
    pub value: String,
}

/// Curation data for JIGS
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigCurationCommentResponse {
    /// ID of comment
    pub id: CommentId,

    /// ID of Jig
    pub jig_id: JigId,

    /// Curator comment
    pub value: String,

    /// When comment was submitted
    pub created_at: Option<DateTime<Utc>>,

    /// ID of commenter
    pub author_id: Uuid,

    /// Name of commenter
    pub author_name: String,
}

into_uuid![CommentId];
