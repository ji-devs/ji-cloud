//! Types to manage image tags.

use crate::domain::meta::TagId;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Request to create an image tag.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageTagCreateRequest {
    /// Display name of the image tag.
    pub display_name: String,
}

/// Response returned to list all image tags.
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageTagListResponse {
    /// Indexes and ids for all the image tags.
    pub image_tags: Vec<ImageTagResponse>,
}

/// Response for a single tag.
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageTagResponse {
    /// The index of the image tag found.
    pub index: i16,

    /// The display name of the image tag found.
    pub display_name: String,

    /// The id of the image tag found.
    pub id: TagId,
}

/// Request to update an image tag.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageTagUpdateRequest {
    /// Display name of the image tag. `None` means no change to be made.
    pub display_name: Option<String>,

    /// If [`Some`] attempt to move tag to the given index. If it is already occupied, do no
    /// change the indexing.
    ///
    /// If `index` is [`None`] then it will not be updated.
    pub index: Option<i16>,
}
