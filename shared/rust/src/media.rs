//! Mostly contains functions for getting the `key`/url of media stored in s3.

use crate::domain::{animation::AnimationKind, audio::AudioKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Media Kinds
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "media_kind")]
#[serde(rename_all = "camelCase")]
pub enum MediaGroupKind {
    /// Media is audio
    Audio,

    /// Media is an image
    Image,

    /// Media is an animation
    Animation,

    /// Media is a pdf
    Pdf,
}

impl MediaGroupKind {
    /// returns `self` in a string representation.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Image => "image",
            Self::Animation => "animation",
            Self::Pdf => "pdf",
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum MediaLibrary {
    /// The default / global library
    Global = 0,

    /// The user library
    User = 1,

    /// The web library
    Web = 2,
}

impl MediaLibrary {
    /// returns `self` in a string representation.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::User => "user",
            Self::Web => "web",
        }
    }
}

impl std::str::FromStr for MediaLibrary {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "global" => Ok(Self::Global),
            "user" => Ok(Self::User),
            "web" => Ok(Self::Web),
            _ => Err(anyhow::anyhow!("media type not recognized")),
        }
    }
}

/// Kinds of media used with the web media library
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum MediaKind {
    /// media is an Animation
    Animation(AnimationKind),

    /// Media is an Image
    Image,

    /// Media is audio
    Audio(AudioKind),

    ///media is a Pdf
    Pdf,
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

    /// File for pdf documents
    DocumentPdf,
}

impl FileKind {
    /// Returns the content type of the represented file
    #[must_use]
    pub const fn content_type(self) -> &'static str {
        match self {
            Self::AnimationGif => "image/gif",
            Self::ImagePng(_) => "image/png",
            Self::AudioMp3 => "audio/mp3",
            Self::DocumentPdf => "application/pdf",
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
            Self::DocumentPdf => "document.pdf",
        }
    }
}

impl std::str::FromStr for FileKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "animation.gif" => Ok(Self::AnimationGif),
            "original.png" => Ok(Self::ImagePng(PngImageFile::Original)),
            "thumbnail.png" => Ok(Self::ImagePng(PngImageFile::Thumbnail)),
            "resized.png" => Ok(Self::ImagePng(PngImageFile::Resized)),
            "audio.mp3" => Ok(Self::AudioMp3),
            "document.pdf" => Ok(Self::DocumentPdf),
            _ => Err(anyhow::anyhow!("media type not recognized")),
        }
    }
}

/// FCM Data Message format for signalling processing completion.
///
/// Contains the information necessary to find the media from the GCS project.
#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct MediaKey {
    /// Media library the
    pub media_library: MediaLibrary,
    /// The id of the media
    pub id: Uuid,
    /// The content type of the media.
    /// The definitions can be found in [`content_type()`](FileKind::content_type).
    pub content_type: String,
}

/// gives the key for some media with the given parameters
/// this is *not* a full url, (for CDN it's missing the domain)
#[must_use]
pub fn media_key(library: MediaLibrary, id: Uuid, file_kind: FileKind) -> String {
    format!(
        "media/{}/{}/{}",
        library.to_str(),
        id.hyphenated(),
        file_kind.suffix()
    )
}
