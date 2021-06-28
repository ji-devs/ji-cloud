use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use super::{
    super::state::Sprite,
    super::super::state::Stickers
};
use shared::domain::jig::module::body::_groups::design::SpriteEffect;

pub fn render(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, sprite: Rc<Sprite>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(stickers, index, sprite => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        Stickers::duplicate(stickers.clone(), index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, index, sprite => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_forward(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, index, sprite => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.move_backward(index);
                    }
                }))
            }),
            /* post-beta 
            html!("menu-line", {
                .property("icon", "crop")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            */
            html!("menu-line", {
                .property("icon", "remove-white")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    sprite.remove_white();
                    stickers.call_change();
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-horizontal")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    sprite.toggle_flip_horizontal();
                    stickers.call_change();
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-vertical")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    sprite.toggle_flip_vertical();
                    stickers.call_change();
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, index => move |evt:events::Click| {
                    sprite.transform.close_menu();
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
