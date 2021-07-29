use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{_groups::design::Sprite, Transform};
use super::{
    super::state::Text,
    super::super::state::{Stickers, Sticker, AsSticker}
};

pub fn render_sticker_text_menu<T: AsSticker>(stickers:Rc<Stickers<T>>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(stickers, index, text => move |evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        Stickers::duplicate(stickers.clone(), index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, index, text => move |evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, index, text => move |evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, text => move |evt:events::Click| {
                    text.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
