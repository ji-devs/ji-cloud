use super::state::*;
use dominator::{clone, html, Dom};
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::prelude::*;

pub fn render_design(state: Rc<ThemeSelector>, slot: Option<&str>, action: Option<Dom>) -> Dom {
    render(None, state, slot, action)
}
pub fn render_cards(state: Rc<ThemeSelector>, slot: Option<&str>, action: Option<Dom>) -> Dom {
    render(Some("cards"), state, slot, action)
}

fn render(
    option_type: Option<&str>,
    state: Rc<ThemeSelector>,
    slot: Option<&str>,
    action: Option<Dom>,
) -> Dom {
    html!("theme-selector", {
        .apply_if(slot.is_some(), |dom| {
            dom.prop("slot", slot.unwrap_ji())
        })
        .apply_if(action.is_some(), |dom| {
            dom.child(html!("empty-fragment", {
                .prop("slot", "action")
                .child(action.unwrap_ji())
            }))
        })
        .children(ThemeId::iter()
          .map(|theme_id| {
            html!("theme-selector-option", {
                .prop("theme", theme_id.as_str_id())
                .prop("optionType", option_type)
                .prop("premium", theme_id.map_theme(|theme| theme).premium)
                .prop_signal("selected", state.selected_signal(theme_id))
                .event(clone!(state => move |_evt:events::Click| {
                    state.set_theme(theme_id);
                }))
            })
          })
        )
    })
}
