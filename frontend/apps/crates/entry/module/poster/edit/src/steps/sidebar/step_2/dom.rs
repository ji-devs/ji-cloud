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

pub struct Step2Dom {}
impl Step2Dom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("module-sidebar-body", {
            .property("slot", "content")
            .child(
                html!("div", {
                    .text("step 2")
                })
            )
        })
    }
}
