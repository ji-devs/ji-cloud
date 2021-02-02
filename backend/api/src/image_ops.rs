use image::{
    gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage, GenericImageView,
    ImageOutputFormat,
};
use shared::domain::{animation::AnimationKind, audio::AudioKind, image::ImageKind};
use shared::media::MediaKind as SharedMediaKind;

/// Kinds of media used with the web media library
#[repr(i16)]
#[derive(Copy, Clone, Debug, sqlx::Type)]
pub enum MediaKind {
    /// Media is a Png, and an Sticker Image
    PngStickerImage = 0,

    /// Media is a Gif, and Animated
    GifAnimation = 1,

    /// Media is a Spritesheet, and Animated
    SpritesheetAnimation = 2,

    /// Media is a Png, and an Canvas Image
    PngCanvasImage = 3,

    /// Media is a Mp3, and Audio
    Mp3Audio = 4,
}

impl MediaKind {
    pub fn to_shared(self) -> SharedMediaKind {
        match self {
            Self::PngStickerImage => SharedMediaKind::Image(ImageKind::Sticker),
            Self::PngCanvasImage => SharedMediaKind::Image(ImageKind::Canvas),
            Self::GifAnimation => SharedMediaKind::Animation(AnimationKind::Gif),
            Self::SpritesheetAnimation => SharedMediaKind::Animation(AnimationKind::Spritesheet),
            Self::Mp3Audio => SharedMediaKind::Audio(AudioKind::Mp3),
        }
    }
}

// todo: use a better method for this
pub fn detect_image_kind(data: &[u8]) -> anyhow::Result<MediaKind> {
    let decoder = GifDecoder::new(&*data);

    let frames = match decoder {
        Ok(decoder) => decoder.into_frames().count(),
        Err(image::ImageError::Decoding(_)) => return Ok(MediaKind::PngStickerImage),
        Err(e) => return Err(e.into()),
    };

    if frames < 2 {
        Ok(MediaKind::PngStickerImage)
    } else {
        Ok(MediaKind::GifAnimation)
    }
}

pub fn regenerate_images(
    original: &DynamicImage,
    kind: ImageKind,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let resized = {
        let (width, height) = kind.size();
        let new_image = match kind {
            ImageKind::Canvas => original.resize_exact(width, height, FilterType::Nearest),

            ImageKind::Sticker if width <= original.width() && height <= original.height() => {
                original.clone()
            }

            ImageKind::Sticker => original.resize(width, height, FilterType::Nearest),
        };

        let mut buffer = Vec::new();
        new_image.write_to(&mut buffer, ImageOutputFormat::Png)?;
        buffer
    };

    let thumbnail = {
        let mut buffer = Vec::new();
        let (width, height) = ImageKind::THUMBNAIL_SIZE;
        original
            .thumbnail(width, height)
            .write_to(&mut buffer, ImageOutputFormat::Png)?;
        buffer
    };

    Ok((resized, thumbnail))
}

pub fn generate_images(
    original: &DynamicImage,
    kind: ImageKind,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let (resized, thumbnail) = regenerate_images(original, kind)?;

    let original = {
        let mut buffer = Vec::new();
        original.write_to(&mut buffer, ImageOutputFormat::Png)?;
        buffer
    };

    Ok((original, resized, thumbnail))
}
