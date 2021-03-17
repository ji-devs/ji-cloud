use dominator::{html, Dom, clone};
use crate::data::*;
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::{SignalVec, SignalVecExt},
};
use super::state::*;
use crate::steps::sidebar::step_1::widgets::single_list::dom::SingleListDom;

pub struct DuplicateDom {}
impl DuplicateDom {
    pub fn render(state:Rc<State>) -> Vec<Dom> { 
        let state = Rc::new(LocalState::new(state));

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
                    SingleListDom::render(state.list.clone())
                ])
            }),
            html!("button-rect", {
                .property("color", "grey")
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "btn")
                .text(crate::strings::STR_DONE)
            })
        ]
    }
}
