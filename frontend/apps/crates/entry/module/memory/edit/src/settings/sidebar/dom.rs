use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::*;
use components::module::_common::edit::settings::prelude::*;
use futures_signals::signal::SignalExt;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    if state.base.extra.settings.use_default_pairs.get() {
        state.set_default_pairs();
    }

    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (
                LineKind::GameDisplay,
                vec![
                    Some(SettingsButton::new_value_click(
                        SettingsButtonKind::NumPairs,
                        clone!(state => move || {
                            state.base.extra.settings.use_default_pairs.signal().map(|v| !v)
                        }),
                        SettingsValue::new_mutable(
                            state.settings().pairs_to_display.clone(),
                            clone!(state => move |value| {
                                state.set_pairs_to_display(value);
                            }),
                        ),
                        clone!(state => move || {
                            state.toggle_use_default_pairs();
                        }),
                    )),
                ]
            ),
            (
                LineKind::TimeLimit,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::TimeLimitOff,
                        clone!(state => move || {
                            state.base.extra.settings.has_time_limit
                                .signal()
                                .map(|flag| !flag)
                        }),
                        clone!(state => move || {
                            state.set_has_time_limit(false);
                        }),
                    )),
                    Some(SettingsButton::new_value_click(
                        SettingsButtonKind::TimeLimit,
                        clone!(state => move || {
                            state.base.extra.settings.has_time_limit
                                .signal()
                        }),
                        SettingsValue::new(
                            state.settings().time_limit.get(),
                            clone!(state => move |value| {
                                state.set_time_limit(value);
                            }),
                        ),
                        clone!(state => move || {
                            state.set_has_time_limit(true);
                        }),
                    )),
                ],
            ),
            // NOTE - not including score until player/jig story is resolved
        ],
    }))
}
