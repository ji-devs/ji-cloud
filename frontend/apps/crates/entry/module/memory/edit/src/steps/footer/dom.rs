use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt
};
use utils::prelude::*;

pub struct FooterDom {}
impl FooterDom {
    pub fn render(state:Rc<State>) -> Dom {

        let game_mode = state.game_mode.get().unwrap_ji();

        html!("module-footer", {
            .property("slot", "footer")
            .child(html!("button-rect", {
                .property("color", "grey")
                .property("size", "small")
                .property("iconAfter", "arrow")
                .property("slot", "btn")
                .text(crate::strings::STR_CONTINUE)
                .event(clone!(state => move |evt:events::Click| {
                    state.next_step();
                }))
            }))
                
        })
    }
}
