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
use crate::steps::sidebar::step_1::widgets::single_list::{
    state::State as ListState,
    dom::SingleListDom,
};

pub struct DuplicateDom {}
impl DuplicateDom {
    pub fn render(state:Rc<State>) -> Dom { 

        let list_state = Rc::new(ListState::new(state, 14));

        html!("module-sidebar-body", {
            .property("slot", "content")
            .child(SingleListDom::render(list_state.clone()))
        })
    }
}
