use dominator::{html, Dom};
use shared::{domain::image::ImageId, media::MediaLibrary};
use wasm_bindgen::prelude::*;
use utils::prelude::*;
pub struct ImageJi {
}

impl ImageJi {
    pub fn render(id: &ImageId, lib: MediaLibrary, slot: Option<&str>) -> Dom {
        html!("img-ji", {
            .property("id", id.0.to_string())
            .property("lib", lib.to_str())
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
        })
    }

    //TODO - FIXME!
    /*
    pub fn render_click(id: &ImageId, lib: MediaLibrary, slot: Option<&str>, on_click: impl FnMut()) -> Dom {
        html!("img-ji", {
            .property("id", id.0.to_string())
            .property("lib", lib.to_str())
            .events(|evt:events::Click| on_click())
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
        })
    }
    */
}
