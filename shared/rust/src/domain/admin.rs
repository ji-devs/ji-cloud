//! Types for admin routes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::media::{MediaKind, MediaLibrary};

/// Query for [`ListMedia`](crate::api::endpoints::admin::ListMedia)
/// Super unstable, may change at any time, for any reason.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "backend", derive(paperclip::actix::Apiv2Schema))]
pub struct AdminListMediaQuery {
    /// Filter to *only* using this specific media kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_kind: Option<MediaKind>,

    /// Filter out anything that was uploaded after this time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploaded_at: Option<DateTime<Utc>>,
}

/// Response for [`ListMedia`](crate::api::endpoints::admin::ListMedia)
/// Super unstable, may change at any time, for any reason.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(paperclip::actix::Apiv2Schema))]
pub struct AdminListMediaResponse {
    /// A list of all media items
    pub media: Vec<AdminMediaItem>,
}

/// Item of media
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(paperclip::actix::Apiv2Schema))]
pub struct AdminMediaItem {
    /// The ID of the media
    pub id: Uuid,

    /// What kind of media this is
    pub kind: MediaKind,

    /// What library this media is from
    pub library: MediaLibrary,

    /// When the media was originally created.
    pub created_at: DateTime<Utc>,

    /// When the media was last updated.
    pub updated_at: Option<DateTime<Utc>>,

    /// When the media was last uploaded.
    pub uploaded_at: Option<DateTime<Utc>>,

    /// An arbitrary (ascii) string representing the current state of the media.
    pub file_etag: Option<String>,
}
