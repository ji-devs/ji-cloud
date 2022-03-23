use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

impl MenuTab {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        html!("menu-tab-with-title", {
            .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
            .property("kind", format!("{}", state.kind))
            .property("disabled", !state.enabled)
            .apply_if(state.sizeable, |dom| {
                dom.property_signal("small", (state.active_signal) ().map(|active| !active))
            })
            .property_signal("active", (state.active_signal) ())
            .event(clone!(state => move |_evt:events::Click| {
                (state.on_click) ();
            }))
        })
    }
}
