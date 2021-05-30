use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::{prelude::*, math::{bounds, transform_signals}};
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use super::{
    state::{Sprite, width_signal, height_signal},
    super::state::Stickers
};
use crate::transform;
use shared::domain::jig::module::body::{Sprite as RawSprite, Transform};

//For stickers, just let the transform affect it directly
//that means it's not a child of the transform, they're independent
//this is both faster for performance, theoretically, and simpler to use the same
//code for playing and editing

pub fn render(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, sprite: Rc<Sprite>) -> Dom {

    html!("empty-fragment", {
        .child(
            html!("img-ji", {
                .visible_signal(sprite.loaded_signal())
                .style_signal("width", sprite.width_signal())
                .style_signal("height", sprite.height_signal())
                // We can just let the full transform take effect
                .style_signal("transform", sprite.transform.denormalize_matrix_string_signal())
                // And pin the coordinate system to the center regardless of screen size
                .style_signal("top", bounds::size_height_center_rem_signal(sprite.transform.size.signal()))
                .style_signal("left", bounds::size_width_center_rem_signal(sprite.transform.size.signal()))
                .style("display", "block")
                .style("position", "absolute")
                .property("id", sprite.id.0.to_string())
                .property("lib", sprite.lib.to_str())
                .property("size", "full")
                .event(clone!(sprite => move |evt:events::ImageLoad| {
                    let size = evt.size();
                    sprite.transform.size.set(Some(size.clone()));

                    /* 
                    if sprite.is_new {
                        sprite.transform.set_to_center();
                    }
                    */
                }))
                .event(clone!(index, stickers => move |evt:events::Click| {
                    if let Some(index) = index.get_cloned() {
                        stickers.select_index(index);
                    }
                }))
            })
        )
        .child_signal(stickers.selected_signal(index.clone()).map(clone!(stickers, sprite, index => move |active| {
            if active {
                Some(transform::dom::render(
                    sprite.transform.clone(),
                    Some(clone!(stickers, index, sprite => move || super::menu::dom::render(stickers.clone(), index.clone(), sprite.clone())))
                ))
            } else {
                None
            }
        })))

    })
}


pub fn render_raw(sprite: &RawSprite) -> Dom {

    let size:Mutable<Option<(f64, f64)>> = Mutable::new(None);

    html!("img-ji", {
        .style("display", "block")
        .style("position", "absolute")
        .visible_signal(size.signal_ref(|size| size.is_some()))
        .style_signal("width", width_signal(size.signal_cloned()))
        .style_signal("height", height_signal(size.signal_cloned()))
        .style_signal("top", bounds::size_height_center_rem_signal(size.signal()))
        .style_signal("left", bounds::size_width_center_rem_signal(size.signal()))
        .style_signal("transform", transform_signals::denormalize_matrix_string(always(sprite.transform.clone())))
        .property("id", sprite.id.0.to_string())
        .property("lib", sprite.lib.to_str())
        .property("size", "full")
        .event(clone!(sprite => move |evt:events::ImageLoad| {
            size.set(Some(evt.size()));
        }))
    })
}
