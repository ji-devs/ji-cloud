//! Types to manage image tags.

use macros::make_path_parts;
use serde::{Deserialize, Serialize};

use crate::{api::endpoints::PathPart, domain::meta::ImageTagIndex};

// i16 is tag index (ImageTag.as_index())
make_path_parts!(ImageTagCreatePath => "/v1/image/tag/{}" => i16);

/// Request to create an image tag.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageTagCreateRequest {
    /// Display name of the image tag.
    pub display_name: String,
}

make_path_parts!(ImageTagListPath => "/v1/image/tag/all");

/// Response returned to list all image tags.
#[derive(Serialize, Deserialize)]
pub struct ImageTagListResponse {
    /// Indices for all the image tags.
    pub image_tags: Vec<ImageTagResponse>,
}

/// Response for a single tag.
#[derive(Serialize, Deserialize)]
pub struct ImageTagResponse {
    /// The index of the image tag found.
    pub index: ImageTagIndex,

    /// The display name of the image tag found.
    pub display_name: String,
}

// i16 is tag index (ImageTag.as_index())
make_path_parts!(ImageTagUpdatePath => "/v1/image/tag/{}" => i16);

/// Request to update an image tag.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageTagUpdateRequest {
    /// Display name of the image tag. `None` means no change to be made.
    pub display_name: Option<String>,

    /// If [`Some`] attempt to move tag to the given index. If it is already occupied, do no
    /// change the indexing.
    ///
    /// If `index` is [`None`] then it will not be updated.
    pub index: Option<ImageTagIndex>,
}

// i16 is tag index (ImageTag.as_index())
make_path_parts!(ImageTagDeletePath => "/v1/image/tag/{}" => i16);
