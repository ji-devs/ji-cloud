//! Types for additional resources for JIGs.

use crate::domain::{
    audio::AudioId, image::ImageId, jig::JigId, learning_path::LearningPathId,
    meta::ResourceTypeId, pdf::PdfId,
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

/// Additional Resources assignment to a JIG or Learning Path
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum JigOrPath {
    /// Additional Resource is assigned to a JIG
    Jig = 0,
    /// Additional Resource is assigned to a Learning Path
    LearningPath = 1,
}

/// Additional Resources assignment to a JIG or Learning Path
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum JigOrPathId {
    /// Additional Resource is assigned to a JIG
    JigId(JigId),
    /// Additional Resource is assigned to a Learning Path
    LearningPathId(LearningPathId),
}
/// Request to create a new `AdditionalResource`.
///
/// [`additional_resource::Create`](crate::api::endpoints::additional_resource::Create)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceCreateRequest {
    /// Display name for additional resource
    pub display_name: String,

    /// JIG Id or Learning Path Id for additional resource
    #[serde(flatten)]
    pub jig_or_path_id: JigOrPathId,

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
    /// Location of Additional Resource by Id
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(flatten)]
    pub jig_or_path_id: Option<JigOrPathId>,

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

/// Request to get `AdditionalResource` associated with a live Jig or Learning Path.
///
/// [`additional_resource::Update`](crate::api::endpoints::additional_resource::GetLive)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceGetLiveQuery {
    /// Location of Additional Resource by Id
    pub jig_or_path: JigOrPath,

    /// Jig Id Or Learning Path Id
    pub id: Uuid,
}

/// Request to get `AdditionalResource` associated with a draft Jig or Learning Path.
///
/// [`additional_resource::AdditionalResourceGetDraftQuery`](crate::api::endpoints::additional_resource::GetDraft)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceGetDraftQuery {
    /// Location of Additional Resource by Id
    pub jig_or_path: JigOrPath,

    /// JIG id Or Learning Path id
    pub id: Uuid,
}

/// Request to delete an `AdditionalResource`.
///
/// [`additional_resource::DeleteAdditionalResourceRequest`](crate::api::endpoints::additional_resource::Delete)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAdditionalResourceRequest {
    /// Location of Additional Resource by Id
    pub jig_or_path: JigOrPath,

    /// JIG id Or Learning Path id
    pub id: Uuid,
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
