use image::{imageops::FilterType, DynamicImage, ImageOutputFormat};
use shared::domain::image::ImageKind;

pub fn generate_images(
    original: DynamicImage,
    kind: ImageKind,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let resized = {
        let mut buffer = Vec::new();
        let (width, height) = kind.size();
        original
            .resize(width, height, FilterType::Nearest)
            .write_to(&mut buffer, ImageOutputFormat::Png)?;
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
