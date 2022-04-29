//! Types for additional resources for Learning Paths.

use crate::domain::{audio::AudioId, image::ImageId, meta::ResourceTypeId, pdf::PdfId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of an additional resource.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AdditionalResourceId(pub Uuid);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Over-the-wire representation of Additional resource.
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
/// [`learning_path::additional_resource::Create`](crate::api::endpoints::learning_path::additional_resource::Create)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceCreateRequest {
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
/// [`learning_path::additional_resource::Update`](crate::api::endpoints::learning_path::additional_resource::Update)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceUpdateRequest {
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

/// Value of additional resource type
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
