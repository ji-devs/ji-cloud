use super::state::State;
use dominator::{clone, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::video::DoneAction;
use std::rc::Rc;

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            ModuleSettingsLine::new(
                LineKind::VideoPlay,
                vec![
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::PlayClick,
                            clone!(state => move || state.base.play_settings.autoplay.signal().map(|autoplay| !autoplay)),
                        )
                        .on_click(clone!(state => move || state.toggle_autoplay()))
                        .build()
                    ),
                    // Some(SettingsButton::new_click(
                    //     SettingsButtonKind::Loop,
                    //     clone!(state => move || {
                    //         state.base.play_settings.done_action.signal_ref(|done_action| {
                    //             matches!(done_action, Some(DoneAction::Loop))
                    //         })
                    //     }),
                    //     clone!(state => move || {
                    //         state.set_unset_next_action(Some(DoneAction::Loop));
                    //     }),
                    // )),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::VideoCaptions,
                            clone!(state => move || state.base.play_settings.captions.signal()),
                        )
                        .on_click(clone!(state => move || state.toggle_captions()))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::Mute,
                            clone!(state => move || state.base.play_settings.muted.signal()),
                        )
                        .on_click(clone!(state => move || state.toggle_muted()))
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
                                state.base.play_settings.done_action.signal_ref(|done_action| {
                                    matches!(done_action, None)
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_unset_next_action(None)))
                        .build()
                    ),
                    Some(
                        SettingsButtonBuilder::new(
                            SettingsButtonKind::ContinueAutomatically,
                            clone!(state => move || {
                                state.base.play_settings.done_action.signal_ref(|done_action| {
                                    matches!(done_action, Some(DoneAction::Next))
                                })
                            }),
                        )
                        .on_click(clone!(state => move || state.set_unset_next_action(Some(DoneAction::Next))))
                        .build()
                    ),
                ],
            ),
        ],
    }))
}
