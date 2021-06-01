use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::{
    prelude::*, 
    image_effects::{self, ImageEffect},
};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use super::{
    state::Sprite,
    super::state::Stickers
};
use shared::{
    domain::jig::module::body::{SpriteEffect, Image},
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


pub async fn load_and_render(image: Image, effects:&[SpriteEffect]) -> (String, f64, f64) {
    let mut effect = ImageEffect::new(image).await;

    for kind in effects.iter() {
        match kind {
            SpriteEffect::RemoveWhite => {
                effect.do_remove_white();
            },
        }
    }

    effect.finalize();


    let url = effect.to_blob_url().await;

    (url, effect.width as f64, effect.height as f64)


}
