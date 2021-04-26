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

        let mode = state.mode.get().unwrap_ji();

        //TODO - simplify with enabled/disabled button element
        //should be able to drive it all via a simple property
        let is_ready = Rc::new(RefCell::new(false));

        html!("module-footer", {
            .future(state.step_ready_signal().for_each(clone!(is_ready => move |ready| {
                *is_ready.borrow_mut() = ready;
                async {}
            })))
            .property("slot", "footer")
            .child(html!("button-rect", {
                .style_signal("pointer-events", state.step_ready_signal().map(|ready| {
                    if ready {
                        "initial"
                    } else {
                        "none"
                    }
                }))
                .property_signal("color", state.step_ready_signal().map(|ready| {
                    if ready {
                        "red"
                    } else {
                        "grey"
                    }
                }))
                .property("size", "small")
                .property("iconAfter", "arrow")
                .property("slot", "btn")
                .text(crate::strings::STR_CONTINUE)
                .event(clone!(state, is_ready => move |evt:events::Click| {
                    if *is_ready.borrow() {
                        state.next_step();
                    }
                }))
            }))
                
        })
    }
}
