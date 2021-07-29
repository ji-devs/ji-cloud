use std::rc::Rc;
use dominator::{clone, Dom};
use shared::domain::jig::module::body::video::DoneAction;
use super::state::State;

use components::module::_common::edit::settings::prelude::*;
pub fn render(state: Rc<State>) -> Dom {
    render_settings(Rc::new(ModuleSettings {
        lines: vec![
            (LineKind::VideoPlay, vec![
                Some(SettingsButton::new_click(
                    SettingsButtonKind::Autoplay, 
                    clone!(state => move || {
                        state.base.play_settings.autoplay.signal()
                    }),
                    clone!(state => move || {
                        state.toggle_autoplay();
                    }),
                )),
                None,
                Some(SettingsButton::new_click(
                    SettingsButtonKind::ContinueNextActivity, 
                    clone!(state => move || {
                        state.base.play_settings.done_action.signal_ref(|done_action| {
                            matches!(done_action, Some(DoneAction::Next))
                        })
                    }),
                    clone!(state => move || {
                        state.set_unset_next_action(Some(DoneAction::Next));
                    }),
                )),
                Some(SettingsButton::new_click(
                    SettingsButtonKind::Loop, 
                    clone!(state => move || {
                        state.base.play_settings.done_action.signal_ref(|done_action| {
                            matches!(done_action, Some(DoneAction::Loop))
                        })
                    }),
                    clone!(state => move || {
                        state.set_unset_next_action(Some(DoneAction::Loop));
                    }),
                )),
            ]),
            (LineKind::VideoFeatures, vec![
                Some(SettingsButton::new_click(
                    SettingsButtonKind::VideoCaptions, 
                    clone!(state => move || {
                        state.base.play_settings.captions.signal()
                    }),
                    clone!(state => move || {
                        state.toggle_captions();
                    }),
                )),
                Some(SettingsButton::new_click(
                    SettingsButtonKind::Mute, 
                    clone!(state => move || {
                        state.base.play_settings.muted.signal()
                    }),
                    clone!(state => move || {
                        state.toggle_muted();
                    }),
                ))
            ]),
        ]
    }))
}
