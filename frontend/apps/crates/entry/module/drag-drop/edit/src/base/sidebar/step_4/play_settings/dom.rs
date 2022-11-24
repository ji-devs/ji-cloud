use dominator::{clone, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use super::state::PlaySettingsState;
use shared::domain::module::body::drag_drop::{Hint, Next};

use components::module::_common::edit::settings::prelude::*;

pub fn render(state: Rc<PlaySettingsState>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (
                LineKind::Next,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::ContinueAll,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::PlaceAll)
                            })
                        }),
                        clone!(state => move || {
                            state.set_next(Next::PlaceAll);
                        }),
                    )),
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::ContinueClick,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::ClickContinue)
                            })
                        }),
                        clone!(state => move || {
                            state.set_next(Next::ClickContinue);
                        }),
                    )),
                ],
            ),
            (
                LineKind::TimeLimit,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::TimeLimitOff,
                        clone!(state => move || {
                            state.base.play_settings.has_time_limit
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
                            state.base.play_settings.has_time_limit
                                .signal()
                        }),
                        SettingsValue::new(
                            state.base.play_settings.time_limit.get(),
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
            (
                LineKind::Hint,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::HighlightOff,
                        clone!(state => move || {
                            state.base.play_settings.hint.signal_ref(|curr| {
                                *curr == Hint::None
                            })
                        }),
                        clone!(state => move || {
                            state.set_hint(Hint::None);
                        }),
                    )),
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::Highlight,
                        clone!(state => move || {
                            state.base.play_settings.hint.signal_ref(|curr| {
                                *curr == Hint::Highlight
                            })
                        }),
                        clone!(state => move || {
                            state.set_hint(Hint::Highlight);
                        }),
                    )),
                ],
            ),
            // Note - not including scoring until player settings is resolved
        ],
    }))
}
