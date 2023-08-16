//! Types for Media.

use crate::api::endpoints::PathPart;
use crate::{DateTime, Utc};
use macros::make_path_parts;
use mymacros::{Deserialize, Serialize};
use crate::Url;
use crate::Uuid;

use crate::media::MediaKind;

make_path_parts!(MediaCreatePath => "/v1/media/image/url");

/// Response for adding a URL to the Web Media Library
#[derive(Serialize, Deserialize, Debug)]
pub struct UrlCreatedResponse {
    /// The ID of the media.
    pub id: Uuid,

    /// What kind of media this was inferred to be.
    pub kind: MediaKind,
}

/// Request for adding a URL to the Web Media Library
#[derive(Serialize, Deserialize, Debug)]
pub struct WebMediaUrlCreateRequest {
    /// The url.
    pub url: Url,
}

make_path_parts!(MediaUrlGetPath => "/v1/media/url/{}" => Url);

make_path_parts!(MediaIdGetPath => "/v1/media/id/{}" => Uuid);

/// Response for getting metadata for media from the web media library.
#[derive(Serialize, Deserialize, Debug)]
pub struct WebMediaMetadataResponse {
    /// The ID of the media
    pub id: Uuid,

    /// What kind of media this is
    pub kind: MediaKind,

    /// The urls associated with this media (can be empty)
    pub urls: Vec<Url>,

    /// When this media was added
    pub created_at: DateTime<Utc>,

    /// When this media was last updated, if ever.
    pub updated_at: Option<DateTime<Utc>>,
}

make_path_parts!(MediaUrlDeletePath => "/v1/media/url/{}" => Url);

make_path_parts!(MediaIdDeletePath => "/v1/media/id/{}" => Uuid);
