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

pub fn render(state:Rc<State>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(state, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "forward")
                .event(clone!(state, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "backward")
                .event(clone!(state, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(state, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
        ])
    })
}
