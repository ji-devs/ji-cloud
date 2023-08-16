//! Types for Course Units

use crate::{
    api::endpoints::PathPart,
    domain::{
        audio::AudioId, image::ImageId, module::body::_groups::design::YoutubeEmbed, pdf::PdfId,
    },
};
use macros::make_path_parts;
use mymacros::{Deserialize, Serialize};

use super::CourseId;

wrap_uuid! {
    /// Wrapper type around [`Uuid`](Uuid), represents the ID of a Course Unit.
    pub struct CourseUnitId
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
/// Unit attached to a Course.
pub struct CourseUnit {
    /// Unit Id
    pub id: CourseUnitId,
    /// Name for Course Unit
    pub display_name: String,

    /// Description of Course Unit
    pub description: String,

    /// Content of Course Unit
    #[serde(flatten)]
    pub value: CourseUnitValue,
}

make_path_parts!(CreateCourseUnitPath => "/v1/course/{}/unit" => CourseId);

/// Request to create a new `CourseUnit`.
///
/// [`course::unit::Create`](crate::api::endpoints::course::unit::Create)
#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct CourseUnitCreateRequest {
    /// Display name for Course Unit
    pub display_name: String,

    /// Type of Course Unit
    pub description: String,

    /// Value of Course Unit
    #[serde(flatten)]
    pub value: CourseUnitValue,
}

make_path_parts!(UpdateCourseUnitPath => "/v1/course/{}/unit/{}" => CourseId, CourseUnitId);

/// Request to update an `CourseUnit`.
///
/// [`course::unit::Update`](crate::api::endpoints::course::unit::Update)
#[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct CourseUnitUpdateRequest {
    /// Course Unit display name
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// Description of Course Unit
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,

    /// Kind of Course Unit
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(flatten)]
    pub value: Option<CourseUnitValue>,

    /// Kind of Course Unit
    #[serde(default)]
    pub index: Option<u16>,
}

make_path_parts!(GetCourseUnitDraftPath => "/v1/course/{}/unit/{}/draft" => CourseId, CourseUnitId);

make_path_parts!(GetCourseUnitLivePath => "/v1/course/{}/unit/{}/live" => CourseId, CourseUnitId);

make_path_parts!(DeleteCourseUnitPath => "/v1/course/{}/unit/{}/draft" => CourseId, CourseUnitId);

/// Value of Course Unit
#[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub enum CourseUnitValue {
    /// Course Unit kind: image
    ImageId(ImageId),
    /// Course Unit kind: audioFile
    AudioId(AudioId),
    /// Course Unit kind: link
    Link(crate::Url),
    /// Course Unit kind: pdf
    PdfId(PdfId),
    /// Course Unit kind: YouTube Video
    Video(YoutubeEmbed),
}
