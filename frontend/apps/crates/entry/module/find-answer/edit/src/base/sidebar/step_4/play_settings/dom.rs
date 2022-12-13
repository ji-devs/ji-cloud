use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::State;
use shared::domain::module::body::find_answer::{Next, Ordering};

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            ModuleSettingsLine::new(
                LineKind::Ordering,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::Randomize,
                            clone!(state => move || {
                                state.base.play_settings.ordering.signal_ref(|curr| {
                                    *curr == Ordering::Randomize
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_ordering(Ordering::Randomize)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::Order,
                            clone!(state => move || {
                                state.base.play_settings.ordering.signal_ref(|curr| {
                                    *curr == Ordering::InOrder
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_ordering(Ordering::InOrder)))
                        .build()
                    ),
                ],
            ),
            // (
            //     LineKind::Attempts,
            //     vec![
            //         Some(SettingsButton::new_click(
            //             SettingsButtonKind::NoLimit,
            //             clone!(state => move || {
            //                 state.base.play_settings.has_attempts_limit.signal_ref(|flag| !flag)
            //             }),
            //             clone!(state => move || {
            //                 state.set_has_attempts_limit(false);
            //             }),
            //         )),
            //         Some(SettingsButton::new_value_click(
            //             SettingsButtonKind::Attempts,
            //             clone!(state => move || {
            //                 state.base.play_settings.has_attempts_limit.signal()
            //             }),
            //             SettingsValue::new(
            //                 state.base.play_settings.n_attempts.get(),
            //                 clone!(state => move |value| {
            //                     state.set_attempts_limit(value);
            //                 }),
            //             ),
            //             clone!(state => move || {
            //                 state.set_has_attempts_limit(true);
            //             }),
            //         )),
            //     ],
            // ),
            ModuleSettingsLine::new(
                LineKind::TimeLimit,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::TimeLimitOff,
                            clone!(state => move || {
                                state.base.play_settings.has_time_limit.signal_ref(|flag| !flag)
                            }),
                        )
                        .on_click(clone!(state => move || state.set_has_time_limit(false)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::TimeLimit,
                            clone!(state => move || {
                                state.base.play_settings.has_time_limit
                                    .signal()
                            }),
                        )
                        .value(SettingsValue::new(
                            state.base.play_settings.time_limit.get(),
                            clone!(state => move |value| {
                                state.set_time_limit(value);
                            }),
                        ))
                        .on_click(clone!(state => move || state.set_has_time_limit(true)))
                        .build()
                    ),
                ],
            ),
            ModuleSettingsLine::new(
                LineKind::Next,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::ContinueClick,
                            clone!(state => move || {
                                state.base.play_settings.next.signal_ref(|curr| {
                                    std::mem::discriminant(curr) == std::mem::discriminant(&Next::Continue)
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_next(Next::Continue)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::ContinueAll,
                            clone!(state => move || {
                                state.base.play_settings.next.signal_ref(|curr| {
                                    std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectAll)
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_next(Next::SelectAll)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::ContinueSome,
                            clone!(state => move || {
                                state.base.play_settings.next.signal_ref(|curr| {
                                    std::mem::discriminant(curr) == std::mem::discriminant(&Next::SelectSome(0))
                                })
                            }),
                        )
                        .value(SettingsValue::new(
                            state.base.play_settings.next_value.get(),
                            clone!(state => move |value| {
                                state.set_next_value(value);
                            }),
                        ))
                        .on_click(clone!(state => move || state.set_next_some()))
                        .build()
                    ),
                ],
            ),
        ],
    }))
}
