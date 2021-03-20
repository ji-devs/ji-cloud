//! Types for JIGs.

pub mod module;

use super::{meta::ContentTypeId, Publish};
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// avoid breaking Changes
pub use module::{LiteModule, ModuleId, ModuleKind};

/// Wrapper type around [`Uuid`], represents the ID of a JIG.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigId(pub Uuid);

/// Special parameter for allowing implicit `me` as a user.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum UserOrMe {
    /// We should use the user found in the session auth.
    Me,

    /// we should use the provided user.
    User(Uuid),
}

/// Request to create a new JIG.
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigCreateRequest {
    /// The JIG's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The JIG's remaining modules.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub modules: Vec<ModuleId>,

    /// The types of content this JIG contains.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub content_types: Vec<ContentTypeId>,

    /// When the JIG should be considered published (if at all).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Publish>,
}

/// The over-the-wire representation of a JIG.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Jig {
    /// The ID of the JIG.
    pub id: JigId,

    /// The JIG's name.
    pub display_name: Option<String>,

    /// The JIG's remaining modules.
    pub modules: Vec<LiteModule>,

    /// The types of content this JIG contains.
    pub content_types: Vec<ContentTypeId>,

    /// The ID of the JIG's original creator ([`None`] if unknown).
    pub creator_id: Option<Uuid>,

    /// The current author
    pub author_id: Option<Uuid>,

    /// When the JIG should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,
}

/// The response returned when a request for `GET`ing a jig is successful.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigResponse {
    /// The requested JIG.
    pub jig: Jig,
}

/// Request for updating a JIG.
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigUpdateRequest {
    /// The JIG's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The JIG's modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub modules: Option<Vec<ModuleId>>,

    /// The types of content this JIG contains.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_types: Option<Vec<ContentTypeId>>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<Uuid>,

    /// When the JIG should be considered published (if at all).
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Option<Publish>>,
}

/// Query for [`Browse`](crate::api::endpoints::jig::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
pub struct JigBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by author id.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserOrMe>,

    /// The page number of the jigs to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}
/// Response for [`Browse`](crate::api::endpoints::jig::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
pub struct JigBrowseResponse {
    /// the jigs returned.
    pub jigs: Vec<Jig>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of jigs found
    pub total_image_count: u64,
}

into_uuid![JigId];
