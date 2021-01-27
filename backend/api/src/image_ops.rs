use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageOutputFormat};
use shared::domain::image::ImageKind;

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
