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
use super::super::state::*;

pub fn render(state:Rc<State>, index: ReadOnlyMutable<Option<usize>>, sticker: Rc<Sticker>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "forward")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "backward")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "crop")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "remove-white")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-horizontal")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "flip-vertical")
                .event(clone!(state, sticker => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(state, index => move |evt:events::Click| {
                    if let Some(index) = index.get() {
                        state.renderables.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
