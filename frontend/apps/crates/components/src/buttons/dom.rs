use super::state::*;
use dominator::{clone, html, Dom};
use std::rc::Rc;

use utils::prelude::*;

impl Button {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        let icon_prop = state.icon_str();

        let label = state.label.as_ref();

        html!(state.element_str(), {
            .apply_if(icon_prop.is_some(), |dom| {
                dom.property("icon", icon_prop.unwrap_ji())
            })
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .apply_if(label.is_some(), |dom| {
                dom.property("label", label.unwrap_ji())
            })
            .apply_if(state.on_click.is_some(), |dom| {
                dom.event(clone!(state => move |_evt:events::Click| {
                    (state.on_click.as_ref().unwrap_ji()) ();
                }))
            })
        })
    }
}
