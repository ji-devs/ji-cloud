//! Types for additional resources for JIG or Playlists.

use crate::{
    api::endpoints::PathPart,
    domain::{asset::AssetId, audio::AudioId, image::ImageId, meta::ResourceTypeId, pdf::PdfId},
};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

wrap_uuid! {
    /// Wrapper type around [`Uuid`](Uuid), represents the ID of an additional resource.
    pub struct AdditionalResourceId("ar")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Over-the-wire representation of a JIG or Playlist additional resource.
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

make_path_parts!(CreateAssetResourcePath => "/v1/additional-resource/draft");

/// Request to create a new `AdditionalResource`.
///
/// [`additional_resource::Create`](crate::api::endpoints::additional_resource::Create)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceCreateRequest {
    /// Asset Id (JIG or Playlist) for additional resource
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

make_path_parts!(UpdateAssetResourcePath => "/v1/additional-resource/{}" => AdditionalResourceId);

/// Request to update an `AdditionalResource`.
///
/// [`additional_resource::Update`](crate::api::endpoints::additional_resource::Update)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceUpdateRequest {
    /// Asset Id (JIG or Playlist) for additional resource
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

make_path_parts!(GetAssetResourceDraftPath => "/v1/additional-resource/{}/draft" => AdditionalResourceId);

make_path_parts!(GetAssetResourceLivePath => "/v1/additional-resource/{}/live" => AdditionalResourceId);

make_path_parts!(DeleteAssetResourcePath => "/v1/additional-resource/{}/draft" => AdditionalResourceId);

/// Delete an `AdditionalResource` by Asset Id.
///
/// [`additional_resource::GetLive`](crate::api::endpoints::additional_resource::GetLive)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetIdResource {
    /// Asset Id (JIG or Playlist) for additional resource
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
