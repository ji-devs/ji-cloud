//! Types for ProDev Units

use crate::{
    api::endpoints::PathPart,
    domain::{
        audio::AudioId, image::ImageId, module::body::_groups::design::YoutubeEmbed, pdf::PdfId,
    },
};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

use super::ProDevId;

wrap_uuid! {
    /// Wrapper type around [`Uuid`](Uuid), represents the ID of a Pro Dev Unit.
    pub struct ProDevUnitId
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Unit attached to a Pro Dev.
pub struct ProDevUnit {
    /// Unit Id
    pub id: ProDevUnitId,
    /// Name for Pro Dev Unit
    pub display_name: String,

    /// Description of Pro Dev Unit
    pub description: String,

    /// Content of Pro Dev Unit
    #[serde(flatten)]
    pub value: ProDevUnitValue,
}

make_path_parts!(CreateProDevUnitPath => "/v1/pro-dev/{}/unit" => ProDevId);

/// Request to create a new `ProDevUnit`.
///
/// [`pro_dev::unit::Create`](crate::api::endpoints::pro_dev::unit::Create)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProDevUnitCreateRequest {
    /// Display name for Pro Dev Unit
    pub display_name: String,

    /// Type of Pro Dev Unit
    pub description: String,

    /// Value of Pro Dev Unit
    #[serde(flatten)]
    pub value: ProDevUnitValue,
}

make_path_parts!(UpdateProDevUnitPath => "/v1/pro-dev/{}/unit/{}" => ProDevId, ProDevUnitId);

/// Request to update an `ProDevUnit`.
///
/// [`pro_dev::unit::Update`](crate::api::endpoints::pro_dev::unit::Update)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProDevUnitUpdateRequest {
    /// Pro Dev Unit display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// Description of Pro Dev Unit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Kind of Pro Dev Unit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(flatten)]
    pub value: Option<ProDevUnitValue>,

    /// Kind of Pro Dev Unit
    #[serde(default)]
    pub index: Option<u16>,
}

make_path_parts!(GetProDevUnitDraftPath => "/v1/pro-dev/{}/unit/{}/draft" => ProDevId, ProDevUnitId);

make_path_parts!(GetProDevUnitLivePath => "/v1/pro-dev/{}/unit/{}/live" => ProDevId, ProDevUnitId);

make_path_parts!(DeleteProDevUnitPath => "/v1/pro-dev/{}/unit/{}/draft" => ProDevId, ProDevUnitId);

/// Value of Pro Dev Unit
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ProDevUnitValue {
    /// Pro Dev Unit kind: image
    ImageId(ImageId),
    /// Pro Dev Unit kind: audioFile
    AudioId(AudioId),
    /// Pro Dev Unit kind: link
    Link(url::Url),
    /// Pro Dev Unit kind: pdf
    PdfId(PdfId),
    /// Pro Dev Unit kind: YouTube Video
    Video(YoutubeEmbed),
}
