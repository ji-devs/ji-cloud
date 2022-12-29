use dominator::{clone, Dom};
use std::rc::Rc;

use super::state::State;
use shared::domain::module::body::find_answer::Ordering;

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            ModuleSettingsLine::new(
                LineKind::Ordering,
                vec![
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
                        .build(),
                    ),
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
                        .build(),
                    ),
                ],
            ),
            ModuleSettingsLine::new_with_label(
                "Highlight correct answer...".into(),
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::custom_kind(
                                SettingsButtonKind::Highlight,
                                "after tries",
                            ),
                            clone!(state => move || {
                                state.base.play_settings.has_attempts_limit.signal()
                            }),
                        )
                        .value(
                            SettingsValue::new(
                                state.base.play_settings.n_attempts.get(),
                                clone!(state => move |value| {
                                    state.set_attempts_limit(value);
                                }),
                            )
                            .value_label_template(ValueLabelTemplate::from((
                                "after", "try", "tries",
                            )))
                            .value_input_kind(InputKind::Field),
                        )
                        .on_click(clone!(state => move || state.set_has_attempts_limit(true)))
                        .build(),
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::HighlightOff,
                            clone!(state => move || {
                                state.base.play_settings.has_attempts_limit.signal_ref(|flag| !flag)
                            }),
                        )
                        .on_click(clone!(state => move || state.set_has_attempts_limit(false)))
                        .build(),
                    ),
                ],
            ),
            ModuleSettingsLine::new_with_label(
                "Would you like to set a time limit per question?".into(),
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::TimeLimitOff,
                            clone!(state => move || {
                                state.base.play_settings.has_time_limit.signal_ref(|flag| !flag)
                            }),
                        )
                        .on_click(clone!(state => move || state.set_has_time_limit(false)))
                        .build(),
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::custom_kind(
                                SettingsButtonKind::TimeLimit,
                                "Time limit (seconds) per question",
                            ),
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
                        .build(),
                    ),
                ],
            ),
        ],
    }))
}
