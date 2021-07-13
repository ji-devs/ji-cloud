use std::rc::Rc;
use dominator::{html, clone, Dom, with_node};
use utils::prelude::*;
use futures_signals::signal::{always, SignalExt};
use crate::base::state::Base;
use shared::domain::jig::module::body::drag_drop::{Hint, Next};
use super::state::PlaySettingsState;

use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<PlaySettingsState>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (LineKind::GameDisplay, vec![
                SettingsButton::new_click(
                    SettingsButtonKind::ContinueClick, 
                    clone!(state => move || {
                        state.base.play_settings.next.signal_ref(|curr| {
                            std::mem::discriminant(curr) == std::mem::discriminant(&Next::ClickContinue)
                        })
                    }),
                    clone!(state => move || {
                        state.set_next(Next::ClickContinue);
                    }),
                ),
                SettingsButton::new_click(
                    SettingsButtonKind::ContinueAll, 
                    clone!(state => move || {
                        state.base.play_settings.next.signal_ref(|curr| {
                            std::mem::discriminant(curr) == std::mem::discriminant(&Next::PlaceAll)
                        })
                    }),
                    clone!(state => move || {
                        state.set_next(Next::PlaceAll);
                    }),
                ),
                
            ]),

            (LineKind::TimeLimit, vec![
                SettingsButton::new_click(
                    SettingsButtonKind::TimeLimitOff, 
                    clone!(state => move || {
                        state.base.play_settings.has_time_limit
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
                        state.base.play_settings.has_time_limit
                            .signal()
                    }),
                    SettingsValue::new(
                        state.base.play_settings.time_limit.get(),
                        clone!(state => move |value| {
                            state.set_time_limit(value);
                        })
                    ),
                    clone!(state => move || {
                        state.set_has_time_limit(true);
                    }),
                ),
            ]),

            (LineKind::GameDisplay, vec![
                SettingsButton::new_click(
                    SettingsButtonKind::Highlight, 
                    clone!(state => move || {
                        state.base.play_settings.hint.signal_ref(|curr| {
                            *curr == Hint::Highlight
                        })
                    }),
                    clone!(state => move || {
                        state.set_hint(Hint::Highlight);
                    }),
                ),
                SettingsButton::new_click(
                    SettingsButtonKind::HighlightOff, 
                    clone!(state => move || {
                        state.base.play_settings.hint.signal_ref(|curr| {
                            *curr == Hint::None
                        })
                    }),
                    clone!(state => move || {
                        state.set_hint(Hint::None);
                    }),
                )
            ]),
            
            // Note - not including scoring until player settings is resolved
        ]
    }))
}
