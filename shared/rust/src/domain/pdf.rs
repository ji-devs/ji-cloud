//! Types for Pdf files.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types for user Pdf library.
pub mod user {
    use serde::{Deserialize, Serialize};

    use super::PdfId;

    /// Response for listing.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserPdfListResponse {
        /// the Pdf files returned.
        pub pdf_files: Vec<UserPdfResponse>,
    }

    /// Response for getting a single Pdf file.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserPdfResponse {
        /// The Pdf file's metadata.
        pub metadata: UserPdf,
    }

    /// Over the wire representation of an Pdf file's metadata.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserPdf {
        /// The Pdf file's ID.
        pub id: PdfId,
        // more fields to be added
    }

    /// Request indicating the size of an image for upload.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserPdfUploadRequest {
        /// The size of the Pdf to be uploaded in bytes. Allows the API server to check that the file size is
        /// within limits and as a verification at GCS that the entire file was uploaded
        pub file_size: usize,
    }

    /// URL to upload an Pdf. Supports resumable uploading.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserPdfUploadResponse {
        /// The session URI used for uploading, including the query for uploader ID
        pub session_uri: String,
    }
}

/// Wrapper type around [`Uuid`](Uuid), represents the ID of an Pdf file.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct PdfId(pub Uuid);

/// Response for getting a single Pdf file.
#[derive(Serialize, Deserialize, Debug)]
pub struct PdfResponse {
    /// The Pdf's metadata.
    pub metadata: PdfMetadata,
}

/// Over the wire representation of an Pdf file's metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct PdfMetadata {
    /// The Pdf's ID.
    pub id: PdfId,

    /// The name of the Pdf.
    pub name: String,

    /// The description of the Pdf file.
    pub description: String,

    /// When the Pdf should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,

    /// When the Pdf was originally created.
    pub created_at: DateTime<Utc>,

    /// When the Pdf was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

into_uuid![PdfId];
