use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use super::{super::Stickers, Sprite};
use crate::transform::dom::TransformDom;

//For stickers, just let the transform affect it directly
//that means it's not a child of the transform, they're independent
//this is both faster for performance, theoretically, and simpler to use the same
//code for playing and editing

pub struct SpriteDom {}
impl SpriteDom {
    pub fn render(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, sprite: Rc<Sprite>) -> Dom {

        html!("empty-fragment", {
            .child(
                html!("img-ji", {
                    .visible_signal(sprite.loaded_signal())
                    .style_signal("width", sprite.width_signal())
                    .style_signal("height", sprite.height_signal())
                    .style_signal("transform", sprite.transform.matrix_string_signal())
                    .style("display", "block")
                    .style("position", "absolute")
                    .style("top", "0")
                    .style("left", "0")
                    .property("id", sprite.id.0.to_string())
                    .property("lib", sprite.lib.to_str())
                    .property("size", "full")
                    .event(clone!(sprite => move |evt:events::ImageLoad| {
                        sprite.transform.size.set(Some(evt.size()));

                        if *sprite.is_new.borrow() {
                            sprite.transform.set_to_center();
                        }
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
                    Some(TransformDom::render(
                        sprite.transform.clone(),
                        clone!(stickers, index, sprite => move || super::menu::dom::render(stickers.clone(), index.clone(), sprite.clone()))
                    ))
                } else {
                    None
                }
            })))

        })
    }
}
