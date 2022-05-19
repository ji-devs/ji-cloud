use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::State;
use shared::domain::module::body::find_answer::{Next, Ordering};

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (
                LineKind::Ordering,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::Randomize,
                        clone!(state => move || {
                            state.base.play_settings.ordering.signal_ref(|curr| {
                                *curr == Ordering::Randomize
                            })
                        }),
                        clone!(state => move || {
                            state.set_ordering(Ordering::Randomize);
                        }),
                    )),
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::Order,
                        clone!(state => move || {
                            state.base.play_settings.ordering.signal_ref(|curr| {
                                *curr == Ordering::InOrder
                            })
                        }),
                        clone!(state => move || {
                            state.set_ordering(Ordering::InOrder);
                        }),
                    )),
                ],
            ),
            (
                LineKind::Attempts,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::NoLimit,
                        clone!(state => move || {
                            state.base.play_settings.has_attempts_limit.signal_ref(|flag| !flag)
                        }),
                        clone!(state => move || {
                            state.set_has_attempts_limit(false);
                        }),
                    )),
                    Some(SettingsButton::new_value_click(
                        SettingsButtonKind::Attempts,
                        clone!(state => move || {
                            state.base.play_settings.has_attempts_limit.signal()
                        }),
                        SettingsValue::new(
                            state.base.play_settings.n_attempts.get(),
                            clone!(state => move |value| {
                                state.set_attempts_limit(value);
                            }),
                        ),
                        clone!(state => move || {
                            state.set_has_attempts_limit(true);
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
                            state.base.play_settings.has_time_limit.signal_ref(|flag| !flag)
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
                LineKind::Next,
                vec![
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::ContinueClick,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::Continue)
                            })
                        }),
                        clone!(state => move || {
                            state.set_next(Next::Continue);
                        }),
                    )),
                    Some(SettingsButton::new_click(
                        SettingsButtonKind::ContinueAll,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectAll)
                            })
                        }),
                        clone!(state => move || {
                            state.set_next(Next::SelectAll);
                        }),
                    )),
                    Some(SettingsButton::new_value_click(
                        SettingsButtonKind::ContinueSome,
                        clone!(state => move || {
                            state.base.play_settings.next.signal_ref(|curr| {
                                std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectSome(0))
                            })
                        }),
                        SettingsValue::new(
                            state.base.play_settings.next_value.get(),
                            clone!(state => move |value| {
                                state.set_next_value(value);
                            }),
                        ),
                        clone!(state => move || {
                            state.set_next_some();
                        }),
                    )),
                ],
            ),
        ],
    }))
}
