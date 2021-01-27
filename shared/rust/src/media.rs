//! Mostly contains functions for getting the `key`/url of media stored in s3.

use crate::domain::{animation::AnimationKind, audio::AudioKind, image::ImageKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Media Kinds
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "media_kind")]
#[serde(rename_all = "camelCase")]
pub enum AlgoliaMediaFilterKind {
    /// Media is audio
    Audio,

    /// Media is an image
    Image,

    /// Media is an animation
    Animation,
}

impl AlgoliaMediaFilterKind {
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
pub enum PngImageFile {
    /// The original image
    Original,

    /// The resized image
    Resized,

    /// A thumbnail of the image
    Thumbnail,
}

/// Media Libraries
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MediaLibrary {
    /// The default / global library
    Global,

    /// The user library
    User,

    /// The web library
    Web,
}

impl MediaLibrary {
    #[must_use]
    const fn to_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::User => "user",
            Self::Web => "web",
        }
    }
}

/// Kinds of media used with the web media library
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "backend", derive(paperclip::actix::Apiv2Schema))]
pub enum MediaKind {
    /// media is an Animation
    Animation(AnimationKind),

    /// Media is an Image
    Image(ImageKind),

    /// Media is audio
    Audio(AudioKind),
    // Audio()
}

/// Kinds of media files
/// FIXME: Really awkward
#[derive(Copy, Clone, Debug)]
pub enum FileKind {
    /// File for an Animated Gif
    AnimationGif,

    /// Files for a PNG Image
    ImagePng(PngImageFile),

    // Spritesheet(Image,JSON),
    /// File for Mp3 audio
    AudioMp3,
}

impl FileKind {
    /// Returns the content type of the represented file
    #[must_use]
    pub const fn content_type(self) -> &'static str {
        match self {
            Self::AnimationGif => "image/gif",
            Self::ImagePng(_) => "image/png",
            Self::AudioMp3 => "audio/mp3",
        }
    }

    #[must_use]
    const fn suffix(self) -> &'static str {
        match self {
            Self::AnimationGif => "animation.gif",
            Self::ImagePng(PngImageFile::Original) => "original.png",
            Self::ImagePng(PngImageFile::Thumbnail) => "thumbnail.png",
            Self::ImagePng(PngImageFile::Resized) => "resized.png",
            Self::AudioMp3 => "audio.mp3",
        }
    }
}

/// gives the key for some media with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
#[must_use]
pub fn media_key(library: MediaLibrary, id: Uuid, file_kind: FileKind) -> String {
    format!(
        "media/{}/{}/{}",
        library.to_str(),
        id.to_hyphenated(),
        file_kind.suffix()
    )
}
