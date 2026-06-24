use utils::image_effects::ImageEffect;
use utils::path::module_image_url;
use utils::prelude::UnwrapJiExt;

use super::state::Sprite;
use shared::{
    domain::{
        image::ImageFileKind,
        module::body::{_groups::design::SpriteEffect, Image},
    },
    media::PngImageFile,
};

impl Sprite {
    pub fn remove_white(&self) {
        let mut effects = self.effects.lock_mut();

        if !effects.contains(&SpriteEffect::RemoveWhite) {
            effects.push(SpriteEffect::RemoveWhite);
        }
    }

    pub fn toggle_flip_horizontal(&self) {
        let mut lock = self.flip_horizontal.lock_mut();

        *lock = !*lock;
    }
    pub fn toggle_flip_vertical(&self) {
        let mut lock = self.flip_vertical.lock_mut();
        *lock = !*lock;
    }
}

pub async fn load_and_render(image: Image, effects: &[SpriteEffect]) -> (String, f64, f64) {
    if image.kind == ImageFileKind::Gif && effects.is_empty() {
        let url = module_image_url(image.lib, image.kind, PngImageFile::Resized, image.id);
        let img = awsm_web::loaders::image::load(url.clone())
            .await
            .unwrap_ji();

        return (url, img.natural_width() as f64, img.natural_height() as f64);
    }

    let mut effect = ImageEffect::new(image, None).await;

    for kind in effects.iter() {
        match kind {
            SpriteEffect::RemoveWhite => {
                effect.do_remove_white();
            }
        }
    }

    effect.finalize();

    let url = effect.to_blob_url().await;

    (url, effect.width as f64, effect.height as f64)
}
