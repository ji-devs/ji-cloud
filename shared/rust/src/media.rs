//! Mostly contains functions for getting the `key`/url of media stored in s3.

use crate::domain::{animation::AnimationId, audio::AudioId, image::ImageId};
use serde::{Deserialize, Serialize};

/// Media Kinds
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaKind {
    /// Media is audio
    Audio,

    /// Media is an image
    Image,

    /// Media is an animation
    Animation,
}

impl MediaKind {
    /// returns `self` in a string representation.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Image => "image",
            Self::Animation => "animation",
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
    #[must_use]
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
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Original => "original",
        }
    }
}

/// Animation Variants
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", derive(paperclip::actix::Apiv2Schema))]
#[repr(i16)]
pub enum AnimationVariant {
    /// Gif Animation
    Gif = 0,
    /// Spritesheet Animation
    Spritesheet = 1,
}

impl AnimationVariant {
    /// returns `self` in a string representation.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Gif => "gif",
            Self::Spritesheet => "spritesheet",
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

    const fn animation_prefix(self) -> &'static str {
        match self {
            Self::Global => "animation/global",
            Self::User => "animation/user",
            Self::Web => "animation/web",
        }
    }
}

/// gives the key for a image with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
#[must_use]
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
#[must_use]
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

/// gives the key for an animation with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
#[must_use]
pub fn animation_id_to_key(
    library_kind: MediaLibraryKind,
    variant: AnimationVariant,
    id: AnimationId,
) -> String {
    format!(
        "{}/{}/{}",
        library_kind.animation_prefix(),
        variant.to_str(),
        id.0.to_hyphenated()
    )
}
