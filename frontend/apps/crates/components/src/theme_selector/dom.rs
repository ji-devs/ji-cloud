use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::ThemeChoice;
use std::rc::Rc;
use utils::prelude::*;

pub fn render_design(state: Rc<ThemeSelector>, slot: Option<&str>) -> Dom {
    render("theme-selector-design-option", state, slot)
}
pub fn render_cards(state: Rc<ThemeSelector>, slot: Option<&str>) -> Dom {
    render("theme-selector-cards-option", state, slot)
}

fn render(element_name: &str, state: Rc<ThemeSelector>, slot: Option<&str>) -> Dom {
    html!("theme-selector", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
        .children(THEME_IDS.iter().copied()
          .map(|theme_id| {
            html!(element_name, {
                .property("theme", theme_id.as_str_id())
                .property_signal("state", state.selected_state_signal(theme_id).map(|selected_state| {
                    match selected_state {
                        SelectedState::Selected => "selected",
                        SelectedState::Jig => "jig",
                        SelectedState::None => "idle"
                    }
                }))
                .event(clone!(state => move |_evt:events::Click| {
                    let theme = {
                        if state.jig_theme_id.get() == theme_id {
                            ThemeChoice::Jig
                        } else {
                            ThemeChoice::Override(theme_id)
                        }
                    };

                    state.set_theme(theme);

                }))
                .child(html!("menu-line", {
                    .property("slot", "menu")
                    .property("icon", "set-jig-theme")
                    .event(clone!(state => move |_evt:events::Click| {
                        state.set_jig_theme_id(theme_id);
                    }))
                }))
            })
          })
        )
    })
}
