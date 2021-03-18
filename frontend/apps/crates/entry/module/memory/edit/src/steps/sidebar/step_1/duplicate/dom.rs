use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::{SignalVec, SignalVecExt},
};
use crate::steps::sidebar::step_1::widgets::single_list::{
    state::State as ListState,
    dom::SingleListDom,
};

pub struct DuplicateDom {}
impl DuplicateDom {
    pub fn render(state:Rc<State>) -> Vec<Dom> { 

        let list_state = Rc::new(ListState::new(14));

        vec![
            html!("step1-sidebar-duplicate", {
                .property("slot", "content")
                .children(&mut [
                    html!("button-text", {
                        .property("slot", "clear")
                        .text(crate::strings::STR_CLEAR)
                    }),
                    html!("button-sidebar", {
                        .property("slot", "input-buttons")
                        .property("mode", "keyboard")
                    }),
                    html!("button-sidebar", {
                        .property("slot", "input-buttons")
                        .property("mode", "dicta")
                    }),
                    html!("button-sidebar", {
                        .property("slot", "input-buttons")
                        .property("mode", "sefaria")
                    }),
                    SingleListDom::render(list_state.clone())
                ])
            }),
            html!("button-rect", {
                .property("color", "grey")
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "btn")
                .text(crate::strings::STR_DONE)
                .event(clone!(state, list_state => move |evt:events::Click| {
                    state.replace_single_list(list_state.derive_list());
                    
                }))
            })
        ]
    }
}
