use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;

pub fn render(state: Rc<Step1>) -> Dom {
    html!("step1-sidebar-container", {
        .children(THEME_IDS.iter().copied()
          .map(|theme_id| {
            html!("step1-sidebar-option", {
                .property("theme", theme_id.as_str_id())
                .property_signal("state", state.base.theme_id.signal().map(clone!(theme_id => move |curr_theme_id| {
                    if curr_theme_id == theme_id {
                        "selected"
                    } else {
                        "idle"
                    }
                })))
                .event(clone!(state => move |evt:events::Click| {
                    state.base.change_theme_id(theme_id);
                }))
            })
          })
        )
    })
}
