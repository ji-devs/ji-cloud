use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::ThemeChoice;
use std::rc::Rc;
use utils::prelude::*;

use crate::dialog::Dialog;

const STR_APPLY_TO_JIG: &str = "Apply to JIG";
const STR_APPLY_TO_MODULE: &str = "Apply to cover only";

pub fn render_design(state: Rc<ThemeSelector>, slot: Option<&str>, action: Option<Dom>) -> Dom {
    render("theme-selector-design-option", state, true, slot, action)
}
pub fn render_cards(state: Rc<ThemeSelector>, slot: Option<&str>, action: Option<Dom>) -> Dom {
    render("theme-selector-cards-option", state, false, slot, action)
}

fn render(
    element_name: &str,
    state: Rc<ThemeSelector>,
    can_apply_to_jig: bool,
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
                            if can_apply_to_jig {
                                state.apply_to_jig_popup_active.set(true);
                            }
                            ThemeChoice::Override(theme_id)
                        }
                    };

                    state.set_theme(theme);
                }))
                // .child(html!("menu-line", {
                //     .property("slot", "menu")
                //     .property("icon", "set-jig-theme")
                //     .event(clone!(state => move |_evt:events::Click| {
                //         state.set_jig_theme_id(theme_id);
                //     }))
                // }))
            })
          })
        )
        .child_signal(state.apply_to_jig_popup_active.signal().map(clone!(state => move |active| {
            match active {
                false => None,
                true => {
                    Some(Dialog::render(
                        clone!(state => move|| {
                            html!("theme-selector-apply-popup", {
                                .children(&mut [
                                    html!("button-rect", {
                                        .property("slot", "actions")
                                        .property("kind", "text")
                                        .property("color", "blue")
                                        .text(STR_APPLY_TO_MODULE)
                                        .event(clone!(state => move |_: events::Click| {
                                            state.apply_to_jig_popup_active.set(false);
                                        }))
                                    }),
                                    html!("button-rect", {
                                        .property("slot", "actions")
                                        .property("kind", "filled")
                                        .property("color", "blue")
                                        .text(STR_APPLY_TO_JIG)
                                        .event(clone!(state => move |_: events::Click| {
                                            state.set_jig_theme_id(state.theme_id.get_cloned());
                                            state.apply_to_jig_popup_active.set(false);
                                        }))
                                    }),
                                ])
                            })
                        }),
                        Some(Box::new(clone!(state => move || {
                            state.apply_to_jig_popup_active.set(false);
                        })))
                    ))
                }
            }
        })))
    })
}
