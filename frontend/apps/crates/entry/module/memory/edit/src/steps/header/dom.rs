use dominator::{html, Dom, clone};
use crate::data::{
    actions,
    state::*,
};
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt
};

use components::module::header::controller::dom::ControllerDom;

pub struct HeaderDom {}
impl HeaderDom {
    pub fn render(state:Rc<State>) -> Dom {

        let game_mode = state.game_mode.get().unwrap_throw();

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
