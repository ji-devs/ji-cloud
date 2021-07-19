//! Types for user image library.

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use super::ImageId;

/// Response for listing.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserImageListResponse {
    /// the images returned.
    pub images: Vec<UserImageResponse>,
}

/// Response for getting a single image.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserImageResponse {
    /// The image metadata.
    pub metadata: UserImage,
}

/// Over the wire representation of an image's metadata.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserImage {
    /// The image's ID.
    pub id: ImageId,
    // more fields to be added
}

/// Request to indicate the size of an user library image for upload.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserImageUploadRequest {
    /// The size of the image to be uploaded in bytes.
    pub file_size: usize,
}

/// URL to upload an user library image, supports resumable uploading.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[cfg_attr(feature = "backend", openapi(empty))]
pub struct UserImageUploadResponse {
    /// The session URI used for uploading, including the query for uploader ID
    pub session_uri: String,
}
