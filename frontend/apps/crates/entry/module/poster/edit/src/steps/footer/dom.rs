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
use std::cell::RefCell;

pub struct FooterDom {}
impl FooterDom {
    pub fn render(state:Rc<State>) -> Dom {

        html!("module-footer", {
            .property("slot", "footer")
            .child(html!("module-footer-continue-button", {
                .property("slot", "btn")
                .property_signal("enabled", state.step_ready_signal())
                .event(clone!(state => move |evt:events::Next| {
                    state.next_step();
                }))
            }))
        })

    }
}
