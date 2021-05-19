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
    super::Sprite,
    super::super::Stickers
};

pub fn render(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, sprite: Rc<Sprite>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
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
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-horizontal")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-vertical")
                .event(clone!(stickers, sprite => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, index => move |evt:events::Click| {
                    if let Some(index) = index.get() {
                        stickers.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
