//! Types for additional resources for JIGs.

use crate::domain::{audio::AudioId, image::ImageId};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of an additional resource.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AdditionalResourceId(pub Uuid);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Over-the-wire representation of a JIG additional resource.
pub struct AdditionalResource {
    /// The additional resources's ID.
    pub id: AdditionalResourceId,

    /// Name for additional resource
    pub display_name: String,

    /// Type of additional resource
    pub resource_type_id: Uuid,

    /// Type of additional resource
    pub resource_content: ResourceContent,
}

/// Request to create a new `AdditionalResource`.
///
/// [`additional_resource::Create`](crate::api::endpoints::jig::additional_resource::Create)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceCreateRequest {
    /// Display name for additional resource
    pub display_name: String,

    /// Type of additional resource
    pub resource_type_id: Uuid,

    /// Value of additional resource
    #[serde(flatten)]
    pub resource_content: ResourceContent,
}

/// Request to update an `AdditionalResource`.
///
/// [`additional_resource::Update`](crate::api::endpoints::jig::additional_resource::Update)
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
    pub resource_type_id: Option<Uuid>,

    /// Kind of additional resource
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(flatten)]
    pub resource_content: Option<ResourceContent>,
}

/// Response for successfully requesting an additional resource.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalResourceResponse {
    /// resource display name
    pub display_name: String,

    /// resource id for resource type
    pub resource_type_id: Uuid,

    /// Value of additional resource
    #[serde(flatten)]
    pub resource_content: ResourceContent,
}

/// Type of additional resource
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Copy)]
#[non_exhaustive]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
#[serde(rename_all = "camelCase")]
pub enum ResourceType {
    /// Additional resource type: activity
    Activity = 0,
    /// Additional resource type: coloring
    Coloring = 1,
    /// Additional resource type: curriculum
    Curriculum = 2,
    /// Additional resource type: craft
    Craft = 3,
    /// Additional resource type: ebook
    EBook = 4,
    /// Additional resource type: flashcards
    Flashcards = 5,
    /// Additional resource type: lessonPlan
    LessonPlan = 6,
    /// Additional resource type: podcast
    Podcast = 7,
    /// Additional resource type: websiteLink
    WebsiteLink = 8,
    /// Additional resource type: worksheet
    Worksheet = 9,
    /// Additional resource type: video
    Video = 10,
}

/// Value of additional resource
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResourceContent {
    /// Additional resource kind: image
    ImageId(ImageId),
    /// Additional resource kind: audioFile
    AudioId(AudioId),
    /// Additional resource kind: link
    Link(url::Url),
}

impl TryFrom<i16> for ResourceType {
    type Error = anyhow::Error;

    fn try_from(i: i16) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Self::Activity),
            1 => Ok(Self::Coloring),
            2 => Ok(Self::Curriculum),
            3 => Ok(Self::Craft),
            4 => Ok(Self::EBook),
            5 => Ok(Self::Flashcards),
            6 => Ok(Self::LessonPlan),
            7 => Ok(Self::Podcast),
            8 => Ok(Self::WebsiteLink),
            9 => Ok(Self::Worksheet),
            10 => Ok(Self::Video),
            _ => anyhow::bail!("Resource kind {} is invalid", i),
        }
    }
}

into_uuid![AdditionalResourceId];
