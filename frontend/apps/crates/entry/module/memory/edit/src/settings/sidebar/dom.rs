use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::state::*;
use super::state::*;
use futures_signals::signal::{SignalExt};
use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (LineKind::TimeLimit, vec![
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
                        })
                    ),
                    clone!(state => move || {
                        state.set_has_time_limit(true);
                    }),
                )),
            ]),


            // NOTE - not including score until player/jig story is resolved
        ]
    }))
}

