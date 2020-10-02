use image::{imageops, DynamicImage, GenericImageView, RgbaImage};
use imageops::FilterType;
use shared::domain::image::ImageKind;

pub fn generate_thumbnail(original: &DynamicImage) -> RgbaImage {
    let (width, height) = ImageKind::THUMBNAIL_SIZE;
    let thumb = original.thumbnail(width, height);
    overlay_centered(&thumb, width, height)
}

pub fn generate_resized(original: &DynamicImage, kind: ImageKind) -> RgbaImage {
    let (width, height) = kind.size();
    let resized = original.resize(width, height, FilterType::Nearest);
    overlay_centered(&resized, width, height)
}

fn overlay_centered(image: &DynamicImage, width: u32, height: u32) -> RgbaImage {
    let (x, y) = ((image.width() - width) / 2, (image.height() - height) / 2);

    let mut background = RgbaImage::new(width, height);

    imageops::overlay(&mut background, image, x, y);
    background
}
