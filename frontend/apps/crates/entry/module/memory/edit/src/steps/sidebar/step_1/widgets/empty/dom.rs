use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
    signal_vec::{SignalVec, SignalVecExt},
};

pub struct EmptyDom {}
impl EmptyDom {
    pub fn render(state:Rc<State>) -> Dom { 
        html!("step1-sidebar-empty", {
            .child(
                html!("button-text", {
                    .property("slot", "clear")
                    .text(crate::strings::STR_CREATE_NEW_LIST)
                    .event(clone!(state => move |evt:events::Click| {
                        state.clear_all();
                    }))
                })
            )
        })
    }
}
