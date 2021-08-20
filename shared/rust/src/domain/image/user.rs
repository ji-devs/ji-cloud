//! Types for user image library.

use serde::{Deserialize, Serialize};

use super::{ImageId, ImageKind};

/// Request for creating a user image profile
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageCreateRequest {
    /// The kind of the image. Most relevant for uploading user profile images
    pub kind: ImageKind,
}

/// Query for listing. This is optional. If used, should be included as part of the query string.
///
/// * `kind` field must match the case as represented in the returned json body (`PascalCase`?).
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageListQuery {
    /// Optionally filter by image kind. If included it will only return results of the corresponding
    /// kinds listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ImageKind>,
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
    /// The image kind
    pub kind: ImageKind,
    // more fields to be added
}

/// Request to indicate the size of an user library image for upload.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageUploadRequest {
    /// The size of the image to be uploaded in bytes.
    pub file_size: usize,
}

/// URL to upload an user library image, supports resumable uploading.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserImageUploadResponse {
    /// The session URI used for uploading, including the query for uploader ID
    pub session_uri: String,
}
