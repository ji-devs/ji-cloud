//! Types for additional resources for JIG or Courses.

use crate::domain::{
    asset::AssetId, audio::AudioId, image::ImageId, meta::ResourceTypeId, pdf::PdfId,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of an additional resource.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AdditionalResourceId(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Over-the-wire representation of a JIG or Course additional resource.
pub struct AdditionalResource {
    /// The additional resources's ID.
    pub id: AdditionalResourceId,

    /// Name for additional resource
    pub display_name: String,

    /// Type of additional resource
    pub resource_type_id: ResourceTypeId,

    /// Content of additional resource
    #[serde(flatten)]
    pub resource_content: ResourceContent,
}

/// Request to create a new `AdditionalResource`.
///
/// [`additional_resource::Create`](crate::api::endpoints::additional_resource::Create)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceCreateRequest {
    /// Asset Id (JIG or Course) for additional resource
    #[serde(flatten)]
    pub asset_id: AssetId,

    /// Display name for additional resource
    pub display_name: String,

    /// Type of additional resource
    pub resource_type_id: ResourceTypeId,

    /// Value of additional resource
    #[serde(flatten)]
    pub resource_content: ResourceContent,
}

/// Request to update an `AdditionalResource`.
///
/// [`additional_resource::Update`](crate::api::endpoints::additional_resource::Update)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceUpdateRequest {
    /// Asset Id (JIG or Course) for additional resource
    #[serde(flatten)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,

    /// resource display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// Type of additional  resource
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resource_type_id: Option<ResourceTypeId>,

    /// Kind of additional resource
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(flatten)]
    pub resource_content: Option<ResourceContent>,
}

/// Delete an `AdditionalResource` by Asset Id.
///
/// [`additional_resource::GetLive`](crate::api::endpoints::additional_resource::GetLive)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetIdResource {
    /// Asset Id (JIG or Course) for additional resource
    #[serde(flatten)]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<AssetId>,
}

/// Value of additional resource
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ResourceContent {
    /// Additional resource kind: image
    ImageId(ImageId),
    /// Additional resource kind: audioFile
    AudioId(AudioId),
    /// Additional resource kind: link
    Link(url::Url),
    /// Additional resource kind: pdf
    PdfId(PdfId),
}

into_uuid![AdditionalResourceId];
