//! Types for audio files.

use super::meta::AudioStyleId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
use uuid::Uuid;

/// Types for user audio library.
pub mod user {
    use serde::{Deserialize, Serialize};

    use super::AudioId;

    /// Response for listing.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserAudioListResponse {
        /// the audio files returned.
        pub audio_files: Vec<UserAudioResponse>,
    }

    /// Response for getting a single audio file.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserAudioResponse {
        /// The audio file's metadata.
        pub metadata: UserAudio,
    }

    /// Over the wire representation of an audio file's metadata.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserAudio {
        /// The audio file's ID.
        pub id: AudioId,
        // more fields to be added
    }

    /// Request indicating the size of an image for upload.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserAudioUploadRequest {
        /// The size of the audio to be uploaded in bytes. Allows the API server to check that the file size is
        /// within limits and as a verification at GCS that the entire file was uploaded
        pub file_size: usize,
    }

    /// URL to upload an audio. Supports resumable uploading.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserAudioUploadResponse {
        /// The session URI used for uploading, including the query for uploader ID
        pub session_uri: String,
    }
}

/// Wrapper type around [`Uuid`](Uuid), represents the ID of an audio file.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AudioId(pub Uuid);

/// Represents different kinds of audio.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum AudioKind {
    /// Audio is an Mp3
    Mp3 = 0,
}

/// Response for getting a single audio file.
#[derive(Serialize, Deserialize, Debug)]
pub struct AudioResponse {
    /// The audio's metadata.
    pub metadata: AudioMetadata,
}

/// Over the wire representation of an audio file's metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct AudioMetadata {
    /// The audio's ID.
    pub id: AudioId,

    /// The name of the audio.
    pub name: String,

    /// The description of the audio file.
    pub description: String,

    /// Is the audio premium?
    pub is_premium: bool,

    /// When the audio should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,

    /// The styles associated with the audio file.
    pub styles: Vec<AudioStyleId>,

    /// What kind of audio this is.
    pub kind: AudioKind,

    /// Should the audio loop?
    pub is_looping: bool,

    /// When the audio was originally created.
    pub created_at: DateTime<Utc>,

    /// When the audio was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

// HACK: can't get `Vec<_>` directly from the DB. See `[crate::domain::image::ImageMetadata]`
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for AudioMetadata {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbAudio {
            id,
            kind,
            name,
            description,
            is_premium,
            publish_at,
            styles,
            is_looping,
            created_at,
            updated_at,
        } = DbAudio::from_row(row)?;

        Ok(Self {
            id,
            kind,
            name,
            description,
            is_premium,
            publish_at,
            styles: styles.into_iter().map(|(it,)| it).collect(),
            is_looping,
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg(feature = "backend")]
struct DbAudio {
    pub id: AudioId,
    pub kind: AudioKind,
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<(AudioStyleId,)>,
    pub is_looping: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

into_uuid![AudioId];
