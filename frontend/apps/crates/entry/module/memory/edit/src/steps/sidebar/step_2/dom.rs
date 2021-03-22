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

pub struct Step2Dom {}
impl Step2Dom {
    pub fn render(state:Rc<State>) -> Vec<Dom> {
        vec![
            html!("step2-sidebar-container", {
                .property("slot", "content")
                .children(crate::config::get_themes_iter()
                  .map(|theme| {
                    html!("step2-sidebar-option", {
                        .property("theme", theme)
                        .property_signal("state", state.theme.signal_ref(clone!(theme => move |curr_theme| {
                            if curr_theme == &theme {
                                "selected"
                            } else {
                                "idle"
                            }
                        })))
                        .event(clone!(state => move |evt:events::Click| {
                            state.theme.set_neq(theme.to_string());
                        }))
                    })
                  })
                )
            })
        ]
    }
}
