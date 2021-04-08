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
    pub fn render(state:Rc<State>) -> Vec<Dom> {
        vec![
            html!("step2-sidebar-container", {
                .property("slot", "content")
                .children(THEME_IDS.iter().copied()
                  .map(|theme_id| {
                    html!("step2-sidebar-option", {
                        .property("theme", theme_id.as_str_id())
                        .property_signal("state", state.theme_id.signal().map(clone!(theme_id => move |curr_theme_id| {
                            if curr_theme_id == theme_id {
                                "selected"
                            } else {
                                "idle"
                            }
                        })))
                        .event(clone!(state => move |evt:events::Click| {
                            state.change_theme_id(theme_id);
                        }))
                    })
                  })
                )
            })
        ]
    }
}
