//! Mostly contains functions for getting the `key`/url of media stored in s3.

use uuid::Uuid;

use crate::domain::{audio::AudioId, image::ImageId};

/// Media Kinds
#[derive(Debug, Clone, Copy)]
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

/// Media Variants
#[derive(Debug, Copy, Clone)]
pub enum MediaVariant {
    /// The original media
    Original,

    /// The resized media (for images)
    Resized,

    /// A thumbnail of the media (for images)
    Thumbnail,
}

impl MediaVariant {
    /// returns `self` in a string representation.
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Resized => "resized",
            Self::Thumbnail => "thumbnail",
        }
    }
}

/// Media Libraries
#[derive(Debug, Copy, Clone)]
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

    const fn prefix(self, media_kind: MediaKind) -> &'static str {
        match media_kind {
            MediaKind::Audio => self.audio_prefix(),
            MediaKind::Image => self.image_prefix(),
        }
    }
}

fn id_to_key_inner(prefix: &str, variant: MediaVariant, id: Uuid) -> String {
    format!("{}/{}/{}", prefix, variant.to_str(), id.to_hyphenated())
}

/// gives the key for a image with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
pub fn image_id_to_key(
    library_kind: MediaLibraryKind,
    variant: MediaVariant,
    id: ImageId,
) -> String {
    id_to_key_inner(library_kind.image_prefix(), variant, id.0)
}

/// gives the key for a audio-file with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
pub fn audio_id_to_key(
    library_kind: MediaLibraryKind,
    variant: MediaVariant,
    id: AudioId,
) -> String {
    id_to_key_inner(library_kind.audio_prefix(), variant, id.0)
}

/// Meant primarily for backend usage
pub fn id_with_kind_to_key(
    library_kind: MediaLibraryKind,
    variant: MediaVariant,
    id: Uuid,
    media_kind: MediaKind,
) -> String {
    id_to_key_inner(library_kind.prefix(media_kind), variant, id)
}
