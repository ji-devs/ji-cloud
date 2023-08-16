//! Types for a user's recent images list. Can be from any ['MediaLibrary'](crate::media::MediaLibrary).
//! Does not verify entries for validity/existence.

use super::ImageId;
use crate::api::endpoints::PathPart;
use crate::media::MediaLibrary;
use crate::{DateTime, Utc};
use macros::make_path_parts;
use mymacros::{Deserialize, Serialize};
use crate::Uuid;

make_path_parts!(UserRecentImageListPath => "/v1/user/me/recent/image");

/// Over-the-wire representation of a single recent image.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecentImageResponse {
    /// The image's ID.
    pub id: ImageId,

    /// The library that the image belongs to.
    pub library: MediaLibrary,

    /// When the image was last used.
    pub last_used: DateTime<Utc>,
}

make_path_parts!(UserRecentImageUpsertPath => "/v1/user/me/recent/image");

/// Request to add an entry to the recent user images list,
/// see ['recent::Put'](crate::api::endpoints::image::recent::Put).
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecentImageUpsertRequest {
    /// The image's ID.
    pub id: ImageId,

    /// The library that the image belongs to.
    pub library: MediaLibrary,
}

/// Query to list a user's recent images,
/// see ['recent::List'](crate::api::endpoints::image::recent::List).
///
/// This query is optional.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecentImageListRequest {
    /// Indicates how many recent items to retrieve.
    pub limit: u16,
}

/// Response for listing a user's recent images,
/// see ['recent::List'](crate::api::endpoints::image::recent::List).
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecentImageListResponse {
    /// The images returned.
    pub images: Vec<UserRecentImageResponse>,
}

// uuid should be sufficient to identify an image, VERY unlikely to conflict across media libraries
make_path_parts!(UserRecentImageDeletePath => "/v1/user/me/recent/image/{}" => Uuid);
