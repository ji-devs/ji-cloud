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
            ModuleSettingsLine::new(
                LineKind::GameDisplay,
                vec![Some(
                    SettingsButtonBuilder::new(
                        SettingsButtonKind::NumPairs,
                        clone!(state => move || {
                            state.base.extra.settings.use_default_pairs.signal().map(|v| !v)
                        }),
                    )
                    .value(SettingsValue::new_mutable(
                        state.settings().pairs_to_display.clone(),
                        clone!(state => move |value| {
                            state.set_pairs_to_display(value);
                        }),
                    ))
                    .on_click(clone!(state => move || state.toggle_use_default_pairs()))
                    .build(),
                )],
            ),
            ModuleSettingsLine::new(
                LineKind::TimeLimit,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::TimeLimitOff,
                            clone!(state => move || {
                                state.base.extra.settings.has_time_limit
                                    .signal()
                                    .map(|flag| !flag)
                            }),
                        )
                        .on_click(clone!(state => move || state.set_has_time_limit(false)))
                        .build(),
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::TimeLimit,
                            clone!(state => move || {
                                state.base.extra.settings.has_time_limit
                                    .signal()
                            }),
                        )
                        .value(SettingsValue::new(
                            state.settings().time_limit.get(),
                            clone!(state => move |value| {
                                state.set_time_limit(value);
                            }),
                        ))
                        .on_click(clone!(state => move || state.set_has_time_limit(true)))
                        .build(),
                    ),
                ],
            ),
            // NOTE - not including score until player/jig story is resolved
        ],
    }))
}
