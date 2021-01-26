//! Types for Media.

use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::media::MediaKind;

/// Response for adding a URL to the Web Media Library
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UrlCreatedResponse {
    /// The ID of the media.
    pub id: Uuid,

    /// What kind of media this was inferred to be.
    pub kind: MediaKind,
}

/// Request for adding a URL to the Web Media Library
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[cfg_attr(feature = "backend", openapi(empty))]
pub struct WebMediaUrlCreateRequest {
    /// The url.
    pub url: Url,
}

/// Response for getting metadata for media from the web media library.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[cfg_attr(feature = "backend", openapi(empty))]
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
