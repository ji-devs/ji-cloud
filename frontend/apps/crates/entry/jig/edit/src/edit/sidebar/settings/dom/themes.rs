use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::{
    events,
    themes::{ThemeIdExt, THEME_IDS},
};

use crate::edit::sidebar::settings::{
    actions::{set_active_popup, update_jig_settings},
    dom::STR_BACK_TO_SETTINGS,
    state::ActiveSettingsPopup,
};

use super::super::state::State;

pub fn render(state: Rc<State>) -> Dom {
    html!("jig-settings-themes", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-rect", {
                .property("kind", "text")
                .property("slot", "back")
                .text(STR_BACK_TO_SETTINGS)
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Main);
                }))
            }),
            html!("button-icon", {
                .property("icon", "x")
                .property("slot", "close")
                .event(clone!(state => move |_:events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            // html!("input-search", {
            //     .property("slot", "search")
            // }),
        ])
        .children(THEME_IDS.iter().copied()
            .map(|theme_id| {
                html!("theme-selector-design-option", {
                    .property("theme", theme_id.as_str_id())
                    .property("hasMenu", false)
                    .property_signal("state", state.theme.signal_cloned().map(clone!(theme_id => move|selected_theme| {
                        if selected_theme == theme_id {
                            "selected"
                        } else {
                            "idle"
                        }
                    })))
                    .event(clone!(state, theme_id => move |_:events::Click| {
                        state.theme.set(theme_id);
                        update_jig_settings(Rc::clone(&state));
                    }))
                })
            }))
    })
}
