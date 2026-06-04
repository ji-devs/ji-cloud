//! Types for user image library.

use crate::api::endpoints::PathPart;
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

use super::{ImageId, ImageSize};

make_path_parts!(UserImageGetPath => "/v1/user/me/image/{}" => ImageId);

make_path_parts!(UserImageCreatePath => "/v1/user/me/image");

/// Request for creating a user image profile
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageCreateRequest {
    /// The size of the image. Most relevant for uploading user profile images
    pub size: ImageSize,
}

make_path_parts!(UserImageListPath => "/v1/user/me/image");

/// Query for listing. This is optional. If used, should be included as part of the query string.
///
/// * `kind` field must match the case as represented in the returned json body (`PascalCase`?).
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageListQuery {
    /// Optionally filter by image kind. If included it will only return results of the corresponding
    /// kinds listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ImageSize>,
}

/// Response for listing.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageListResponse {
    /// the images returned.
    pub images: Vec<UserImageResponse>,
}

/// Response for getting a single image.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageResponse {
    /// The image metadata.
    pub metadata: UserImage,
}

/// Over the wire representation of an image's metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImage {
    /// The image's ID.
    pub id: ImageId,
    /// The image size
    pub size: ImageSize,
    // more fields to be added
}

make_path_parts!(UserImageUploadPath => "/v1/user/me/image/{}/raw" => ImageId);

make_path_parts!(UserImageDeletePath => "/v1/user/me/image/{}" => ImageId);
