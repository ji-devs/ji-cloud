use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use super::{
    nav::dom::StepsNavDom,
    step_1::dom::Step1Dom,
};
use futures_signals::{
    map_ref,
    signal::SignalExt
};

pub struct SidebarDom {}
impl SidebarDom {
    pub fn render(state:Rc<State>) -> Dom {

        let game_mode = state.game_mode.get().unwrap_throw();

        html!("module-sidebar", {
            .property("slot", "sidebar")
            .child(StepsNavDom::render(state.clone()))
            .children_signal_vec(
                state.step
                    .signal()
                    .switch_signal_vec(clone!(state => move |step| {
                        match step {
                            Step::One => Step1Dom::render(state.clone()),
                            _ => {
                                unimplemented!("can't handle step 2 yet!"); 
                            }
                        }
                    }))
            )
        })
    }
}
