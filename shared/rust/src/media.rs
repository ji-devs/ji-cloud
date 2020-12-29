//! Mostly contains functions for getting the `key`/url of media stored in s3.

use crate::domain::{audio::AudioId, image::ImageId};
use serde::{Deserialize, Serialize};

/// Media Kinds
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaKind {
    /// Media is audio
    Audio,

    /// Media is an image
    Image,
}

impl MediaKind {
    /// returns `self` in a string representation.
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Image => "image",
        }
    }
}

/// Image size Variants
#[derive(Debug, Copy, Clone)]
pub enum ImageVariant {
    /// The original image
    Original,

    /// The resized image
    Resized,

    /// A thumbnail of the image
    Thumbnail,
}

impl ImageVariant {
    /// returns `self` in a string representation.
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Resized => "resized",
            Self::Thumbnail => "thumbnail",
        }
    }
}

/// Audio Variants - for now just one but could add more later
#[derive(Debug, Copy, Clone)]
pub enum AudioVariant {
    /// The original audio
    Original,
}

impl AudioVariant {
    /// returns `self` in a string representation.
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Original => "original",
        }
    }
}

/// Media Libraries
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MediaLibraryKind {
    /// The default / global library
    Global,

    /// The user library
    User,

    /// The web library
    Web,
}

impl MediaLibraryKind {
    const fn image_prefix(self) -> &'static str {
        match self {
            Self::Global => "image",
            Self::User => "image-user",
            Self::Web => "image-web",
        }
    }

    const fn audio_prefix(self) -> &'static str {
        match self {
            Self::Global => "audio/global",
            Self::User => "audio/user",
            Self::Web => "audio/web",
        }
    }
}

/// gives the key for a image with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
pub fn image_id_to_key(
    library_kind: MediaLibraryKind,
    variant: ImageVariant,
    id: ImageId,
) -> String {
    format!(
        "{}/{}/{}",
        library_kind.image_prefix(),
        variant.to_str(),
        id.0.to_hyphenated()
    )
}

/// gives the key for a audio-file with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
pub fn audio_id_to_key(
    library_kind: MediaLibraryKind,
    variant: AudioVariant,
    id: AudioId,
) -> String {
    format!(
        "{}/{}/{}",
        library_kind.audio_prefix(),
        variant.to_str(),
        id.0.to_hyphenated()
    )
}
