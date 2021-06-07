//! Types for a user's recent images list. Can be from any ['MediaLibrary'](crate::media::MediaLibrary).
//! Does not verify entries for validity/existence.

use super::ImageId;
use crate::media::MediaLibrary;
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Over-the-wire representation of a single recent image.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserRecentImageResponse {
    /// The image's ID.
    pub id: ImageId,

    /// The library that the image belongs to.
    pub library: MediaLibrary,

    /// When the image was last used.
    pub last_used: DateTime<Utc>,
}

/// Request to add an entry to the recent user images list,
/// see ['recent::Create'](crate::endpoints::image::recent::Create).
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserRecentImageCreateRequest {
    /// The image's ID.
    pub id: ImageId,

    /// The library that the image belongs to.
    pub library: MediaLibrary,
}

/// Query to list a user's recent images,
/// see ['recent::List'](crate::endpoints::image::recent::List).
///
/// This query is optional.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserRecentImageListRequest {
    /// Indicates how many recent items to retrieve.
    pub limit: u16,
}

/// Response for listing a user's recent images,
/// see ['recent::List'](crate::endpoints::image::recent::List).
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UserRecentImageListResponse {
    /// The images returned.
    pub images: Vec<UserRecentImageResponse>,
}
