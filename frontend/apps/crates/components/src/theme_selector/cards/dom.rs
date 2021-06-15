use dominator::{clone, html, Dom};
use std::rc::Rc;
use super::super::state::*;
use utils::prelude::*;
use shared::domain::jig::module::body::{ThemeChoice, ThemeId};
use futures_signals::signal::{Signal, SignalExt};

pub fn render(state: Rc<ThemeSelector>, slot: Option<&str>) -> Dom {
    html!("theme-cards-selector", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
        .children(THEME_IDS.iter().copied()
          .map(|theme_id| {
            html!("theme-cards-selector-option", {
                .property("theme", theme_id.as_str_id())
                .property_signal("state", state.selected_state_signal(theme_id).map(|selected_state| {
                    match selected_state {
                        SelectedState::Selected => "selected",
                        SelectedState::Jig => "jig",
                        SelectedState::None => "idle"
                    }
                }))
                .event(clone!(state => move |evt:events::Click| {
                    let theme = {
                        if state.jig_theme_id.get() == theme_id {
                            ThemeChoice::Jig
                        } else {
                            ThemeChoice::Override(theme_id)
                        }
                    };

                    state.set_theme(theme);

                }))                    
                .child(html!("menu-kebab", {
                    .property("slot", "menu")
                    .child(html!("menu-line", {
                        .property("icon", "set-jig-theme")
                        .event(clone!(state => move |evt:events::Click| {
                            state.set_jig_theme_id(theme_id);
                        }))                    
                    }))
                }))
            })
          })
        )
    })
}
