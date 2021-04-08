use dominator::{html, Dom, clone};
use crate::data::{
    actions,
    state::*,
};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt
};
use crate::steps::sidebar::nav::dom::StepsNavDom;

use components::module::header::controller::dom::ControllerDom;

pub struct HeaderDom {}
impl HeaderDom {
    pub fn render(state:Rc<State>) -> Dom {

        let game_mode = state.game_mode.get().unwrap_ji();

        html!("header-memory", {
            .property("slot", "header")
            .child(ControllerDom::render(
                state.history.clone(),
                clone!(state => move |history| {
                    state.set_from_history(history);
                })
            ))
            .child(html!("header-button-add", {
                .property("slot", "button")
                .event(clone!(state => move |evt:events::Click| {
                    state.add_card(); 
                }))
            }))
        })
    }
}

pub struct HeaderPreviewDom {}
impl HeaderPreviewDom {
    pub fn render(state:Rc<State>) -> Dom {


        html!("module-preview-header", {
            .property("slot", "header")
            .child(StepsNavDom::render(state.clone()))
            .child(html!("button-rect", {
                .property("slot", "btn")
                .property("size", "small")
                .property("iconAfter", "arrow")
                .text(crate::strings::STR_DONE)
                .event(clone!(state => move |evt:events::Click| {
                }))
            }))
        })
    }
}
