use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{Sprite, Transform};
use super::state::*;
use components::transform::{
    dom::TransformDom,
};

//For stickers, just let the transform affect it directly
//that means it's not a child of the transform, they're independent
//this is both faster for performance, theoretically, and simpler to use the same
//code for playing and editing

pub struct StickerDom {}
impl StickerDom {
    pub fn render(state:Rc<State>, index: ReadOnlyMutable<Option<usize>>, sticker: Rc<Sticker>) -> Dom {

        html!("empty-fragment", {
            .child(
                html!("img-ji", {
                    .visible_signal(sticker.loaded_signal())
                    .style_signal("width", sticker.width_signal())
                    .style_signal("height", sticker.height_signal())
                    .style_signal("transform", sticker.transform.matrix_string_signal())
                    .style("display", "block")
                    .style("position", "absolute")
                    .style("top", "0")
                    .style("left", "0")
                    .property("id", sticker.id.0.to_string())
                    .property("lib", sticker.lib.to_str())
                    .property("size", "full")
                    .event(clone!(sticker => move |evt:events::ImageLoad| {
                        sticker.transform.size.set(Some(evt.size()));

                        if *sticker.is_new.borrow() {
                            sticker.transform.set_to_center();
                        }
                    }))
                    .event(clone!(index, state => move |evt:events::Click| {
                        if let Some(index) = index.get_cloned() {
                            state.select_renderable(index);
                        }
                    }))
                })
            )
            .child_signal(state.renderables.selected_signal(index.clone()).map(clone!(state, sticker, index => move |active| {
                if active {
                    Some(TransformDom::render(
                        sticker.transform.clone(),
                        clone!(state, index, sticker => move || super::menu::dom::render(state.clone(), index.clone(), sticker.clone()))
                    ))
                } else {
                    None
                }
            })))

        })
    }
}
