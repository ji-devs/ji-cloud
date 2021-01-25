use image::{
    gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage, GenericImageView,
    ImageOutputFormat,
};
use shared::media::WebMediaKind as SharedWebMediaKind;
use shared::{domain::image::ImageKind, media::AnimationVariant};

/// Kinds of media used with the web media library
#[repr(i16)]
#[derive(Copy, Clone, Debug, sqlx::Type)]
pub enum WebMediaKind {
    /// Media is a Png, and an Image
    PngStickerImage = 0,

    /// Media is a Gif, and Animated
    GifAnimation = 1,
}

impl WebMediaKind {
    pub fn to_shared(self) -> SharedWebMediaKind {
        match self {
            Self::PngStickerImage => SharedWebMediaKind::Image(ImageKind::Sticker),
            Self::GifAnimation => SharedWebMediaKind::Animation(AnimationVariant::Gif),
        }
    }
}

// todo: use a better method for this
pub fn detect_image_kind(data: &[u8]) -> anyhow::Result<WebMediaKind> {
    let decoder = GifDecoder::new(&*data);

    let frames = match decoder {
        Ok(decoder) => decoder.into_frames().count(),
        Err(image::ImageError::Decoding(_)) => return Ok(WebMediaKind::PngStickerImage),
        Err(e) => return Err(e.into()),
    };

    if frames < 2 {
        Ok(WebMediaKind::PngStickerImage)
    } else {
        Ok(WebMediaKind::GifAnimation)
    }
}

pub fn generate_images(
    original: &DynamicImage,
    kind: ImageKind,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let resized = {
        let (width, height) = kind.size();
        let new_image = match kind {
            ImageKind::Canvas => original.resize_exact(width, height, FilterType::Nearest),

            ImageKind::Sticker if original.width() >= width && original.height() >= height => {
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

    let original = {
        let mut buffer = Vec::new();
        original.write_to(&mut buffer, ImageOutputFormat::Png)?;
        buffer
    };

    Ok((original, resized, thumbnail))
}
