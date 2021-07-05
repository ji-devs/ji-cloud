use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};

use crate::traces::edit::state::*;

pub fn render_menu(state:Rc<Edit>, index: ReadOnlyMutable<Option<usize>>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "edit")
                .event(clone!(state, index => move |evt:events::Click| {
                    if let Some(index) = index.get() {
                        Edit::start_draw(state.clone(), Some(index), None);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(state, index => move |evt:events::Click| {
                    if let Some(index) = index.get() {
                        state.duplicate(index);
                    }
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(state, index => move |evt:events::Click| {
                    if let Some(index) = index.get() {
                        state.delete_index(index);
                    }
                }))
            }),
        ])
    })
}
