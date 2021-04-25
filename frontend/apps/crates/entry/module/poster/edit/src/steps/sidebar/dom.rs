use dominator::{html, Dom, clone};
use crate::data::state::*;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::SignalExt,
};
use crate::steps::nav::dom::StepsNavDom;

pub struct SidebarDom {
}

impl SidebarDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("module-sidebar", {
            .property("slot", "sidebar")
            .child(StepsNavDom::render(state.clone()))
            .children_signal_vec(
                state.step
                    .signal()
                    .map(clone!(state => move |step| {
                        vec![
                            html!("module-sidebar-body", {
                                .property("slot", "content")
                                .children(&mut [
                                    html!("div", {
                                        .text(&format!("STEP {:?} HERE", step))
                                    })
                                ])
                            }),
                        ]
                    }))
                    .to_signal_vec()
            )
        })
    }
}
