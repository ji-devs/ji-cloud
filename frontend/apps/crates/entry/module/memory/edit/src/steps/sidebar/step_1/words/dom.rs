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
use crate::steps::sidebar::step_1::widgets::{
    dual_list::{
        state::State as DualListState,
        dom::DualListDom,
    },
    single_list::{
        state::State as SingleListState,
        dom::SingleListDom,
    },
    empty::dom::EmptyDom
};

pub struct WordsDom {}
impl WordsDom {
    pub fn render(state:Rc<State>, is_empty: bool, is_dual: bool) -> Dom { 


        html!("module-sidebar-body", {
            .property("slot", "content")
            .child({
                if is_empty {
                    EmptyDom::render(state)
                } else {
                    if is_dual {
                        let list_state = Rc::new(DualListState::new(state, 14));
                        DualListDom::render(list_state.clone())
                    } else {
                        let list_state = Rc::new(SingleListState::new(state, 14));
                        SingleListDom::render(list_state.clone())
                    }
                }
            })
        })
    }
}
