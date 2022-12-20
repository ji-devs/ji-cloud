use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::*;
use futures_signals::signal::{always, SignalExt};

use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            ModuleSettingsLine::new(
                LineKind::GameDisplay,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::Swap,
                            clone!(state => move || {
                                state.base.extra.settings.swap.signal()
                            }),
                        )
                        .on_click(clone!(state => move || state.toggle_swap()))
                        .build(),
                    ),
                    Some(
                        SettingsButtonBuilder::new(SettingsButtonKind::NumPairsAlt, || {
                            always(true)
                        })
                        .value(SettingsValue::new(
                            state.settings().n_choices.get(),
                            clone!(state => move |value| {
                                state.set_n_choices(value);
                            }),
                        ))
                        .build(),
                    ),
                ],
            ),
            ModuleSettingsLine::new(
                LineKind::Rounds,
                vec![Some(
                    SettingsButtonBuilder::new(SettingsButtonKind::Rounds, || always(true))
                        .value(SettingsValue::new(
                            state.settings().n_rounds.get(),
                            clone!(state => move |value| {
                                state.set_n_rounds(value);
                            }),
                        ))
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
