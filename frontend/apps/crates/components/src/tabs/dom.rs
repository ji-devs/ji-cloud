use dominator::{html, Dom, clone};
use super::state::*;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;

impl MenuTab {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        html!("menu-tab-with-title", {
            .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
            .property("kind", state.kind.as_str())
            .apply_if(state.sizeable, |dom| {
                dom.property_signal("small", (state.active_signal) ().map(|active| !active)) 
            })
            .property_signal("active", (state.active_signal) ()) 
            .event(clone!(state => move |evt:events::Click| {
                (state.on_click) ();
            }))
        })
    }
}
