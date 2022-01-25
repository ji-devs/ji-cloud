use super::state::*;
use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

pub fn render_design(state: Rc<ThemeSelector>, slot: Option<&str>, action: Option<Dom>) -> Dom {
    render("theme-selector-design-option", state, slot, action)
}
pub fn render_cards(state: Rc<ThemeSelector>, slot: Option<&str>, action: Option<Dom>) -> Dom {
    render("theme-selector-cards-option", state, slot, action)
}

fn render(
    element_name: &str,
    state: Rc<ThemeSelector>,
    slot: Option<&str>,
    action: Option<Dom>
) -> Dom {
    html!("theme-selector", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
        .apply_if(action.is_some(), |dom| {
            dom.child(html!("empty-fragment", {
                .property("slot", "action")
                .child(action.unwrap_ji())
            }))
        })
        .children(THEME_IDS.iter().copied()
          .map(|theme_id| {
            html!(element_name, {
                .property("theme", theme_id.as_str_id())
                .property_signal("selected", state.selected_signal(theme_id))
                .event(clone!(state => move |_evt:events::Click| {
                    state.set_theme(theme_id);
                }))
            })
          })
        )
    })
}
