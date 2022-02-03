//! Types for admin routes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::media::{MediaKind, MediaLibrary};

/// Response for [`ListMedia`](crate::api::endpoints::admin::ListMedia)
/// Super unstable, may change at any time, for any reason.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminListMediaResponse {
    /// A list of all media items
    pub media: Vec<AdminMediaItem>,
}

/// Item of media
#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// Type of data export to perform
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    /// Export user profiles
    Profiles,
}

/// Request to export data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportDataRequest {
    /// The type of data to export
    pub export_type: ExportType,
    /// Optionally the date to export data from
    pub from_date: Option<chrono::DateTime<Utc>>,
    /// Optionally the date to export data to
    pub to_date: Option<chrono::DateTime<Utc>>,
}
