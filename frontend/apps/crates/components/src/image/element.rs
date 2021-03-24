use dominator::{html, Dom};
use shared::{domain::image::ImageId, media::MediaLibrary};
use wasm_bindgen::prelude::*;

pub struct ImageJi {
}

impl ImageJi {
    pub fn render(id: &ImageId, lib: MediaLibrary, slot: Option<&str>) -> Dom {
        html!("img-ji", {
            .property("id", id.0.to_string())
            .property("lib", lib.to_str())
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_throw())
            })
        })
    }
}
