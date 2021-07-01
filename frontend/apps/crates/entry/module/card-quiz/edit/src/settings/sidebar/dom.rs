use dominator::{html, Dom, clone, with_node};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use utils::prelude::*;
use futures_signals::signal::{always, SignalExt};
use components::module::_groups::cards::lookup::Side;
use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (LineKind::GameDisplay, vec![
                SettingsButton::new_value(
                    SettingsButtonKind::NumChoices,
                    || always(true),
                    SettingsValue::new(
                        state.settings().n_choices.get(),
                        clone!(state => move |value| {
                            state.set_n_choices(value);
                        })
                    )
                ),
                SettingsButton::new_click(
                    SettingsButtonKind::Swap, 
                    clone!(state => move || {
                        state.base.extra.settings.swap.signal()
                    }),
                    clone!(state => move || {
                        state.toggle_swap();
                    }),

                )
            ]),

            (LineKind::Rounds, vec![
                SettingsButton::new_value(
                    SettingsButtonKind::Rounds,
                    || always(true),
                    SettingsValue::new(
                        state.settings().n_rounds.get(),
                        clone!(state => move |value| {
                            state.set_n_rounds(value);
                        })
                    )
                ),
            ]),

            (LineKind::TimeLimit, vec![
                SettingsButton::new_click(
                    SettingsButtonKind::TimeLimitOff, 
                    clone!(state => move || {
                        state.base.extra.settings.has_time_limit
                            .signal()
                            .map(|flag| !flag) 
                    }),
                    clone!(state => move || {
                        state.set_has_time_limit(false);
                    }),
                ),
                SettingsButton::new_value_click(
                    SettingsButtonKind::TimeLimit,
                    clone!(state => move || {
                        state.base.extra.settings.has_time_limit
                            .signal()
                    }),
                    SettingsValue::new(
                        state.settings().time_limit.get(),
                        clone!(state => move |value| {
                            state.set_time_limit(value);
                        })
                    ),
                    clone!(state => move || {
                        state.set_has_time_limit(true);
                    }),
                ),
            ]),

            (LineKind::Attempts, vec![
                SettingsButton::new_click(
                    SettingsButtonKind::NoLimit, 
                    clone!(state => move || {
                        state.base.extra.settings.has_attempts_limit
                            .signal()
                            .map(|flag| !flag) 
                    }),
                    clone!(state => move || {
                        state.set_has_attempts_limit(false);
                    }),
                ),
                SettingsButton::new_value_click(
                    SettingsButtonKind::Attempts,
                    clone!(state => move || {
                        state.base.extra.settings.has_attempts_limit
                            .signal()
                    }),
                    SettingsValue::new(
                        state.settings().attempts_limit.get(),
                        clone!(state => move |value| {
                            state.set_attempts_limit(value);
                        })
                    ),
                    clone!(state => move || {
                        state.set_has_attempts_limit(true);
                    }),
                ),
            ]),

            // NOTE - not including score until player/jig story is resolved
        ]
    }))
}
