//! Types for audio files.

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types for user audio library.
pub mod user {
    #[cfg(feature = "backend")]
    use paperclip::actix::Apiv2Schema;

    use serde::{Deserialize, Serialize};

    use super::AudioId;

    /// Response for listing.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserAudioListResponse {
        /// the audio files returned.
        pub audio_files: Vec<UserAudioResponse>,
    }

    /// Response for getting a single audio file.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserAudioResponse {
        /// The audio file's metadata.
        pub metadata: UserAudio,
    }

    /// Over the wire representation of an audio file's metadata.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserAudio {
        /// The audio file's ID.
        pub id: AudioId,
        // more fields to be added
    }
}

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a image.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AudioId(pub Uuid);

into_uuid![AudioId];
