use std::rc::Rc;

use components::audio::mixer::{AudioHandle, AudioPath, AUDIO_MIXER};
use dominator::{clone, html, with_node, Dom};
use web_sys::HtmlInputElement;

use crate::edit::sidebar::jig::settings::{
    actions::{self, set_active_popup},
    dom::STR_BACK_TO_SETTINGS,
    state::ActiveSettingsPopup,
};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::jig::AudioBackground;
use utils::{asset::JigAudioExt, events};

use super::super::state::JigSettings;

impl JigSettings {
    pub fn render_background(self: &Rc<Self>) -> Dom {
        let state = self;

        let current_audio_index: Mutable<Option<usize>> = Mutable::new(None);
        let current_audio_handle: Mutable<Option<AudioHandle>> = Mutable::new(None);

        html!("jig-audio-body", {
            .prop("slot", "overlay")
            .prop("kind", "background")
            .children(&mut [
                html!("button-rect", {
                    .prop("kind", "text")
                    .prop("slot", "back")
                    .prop("color", "blue")
                    .child(html!("fa-icon", {.prop("icon", "fa-light fa-chevron-left")}))
                    .text(STR_BACK_TO_SETTINGS)
                    .event(clone!(state => move|_: events::Click| {
                        set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Main);
                    }))
                }),
                html!("button-icon", {
                    .prop("icon", "x")
                    .prop("slot", "close")
                    .event(clone!(state => move |_:events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                // html!("input-search", {
                //     .prop("slot", "search")
                // }),
            ])
            .children(AudioBackground::variants().iter().enumerate().map(clone!(state, current_audio_index, current_audio_handle => move|(index, option)| {
                state.background_line(option, current_audio_index.clone(), current_audio_handle.clone(), index)
            })).collect::<Vec<Dom>>())
            .after_removed(clone!(current_audio_index, current_audio_handle => move |_| {
                current_audio_index.set(None);
                current_audio_handle.set(None);
            }))
        })
    }

    fn background_line(
        self: &Rc<Self>,
        option: &AudioBackground,
        audio_index: Mutable<Option<usize>>,
        audio_handle: Mutable<Option<AudioHandle>>,
        index: usize,
    ) -> Dom {
        let state = self;

        html!("jig-audio-line", {
            .prop("slot", "lines")
            .prop("label", option.display_name())
            .prop_signal("playing", audio_index.signal_ref(move |audio_index| {
                match audio_index {
                    Some(audio_index) if *audio_index == index => true,
                    _ => false,
                }
            }))
            .children(&mut [
                html!("input" => HtmlInputElement, {
                    .with_node!(elem =>{
                        .prop("slot", "checkbox")
                        .prop("type", "checkbox")
                        .prop_signal("checked", state.jig.audio_background.signal_cloned().map(clone!(option => move|selected_audio| {
                            match selected_audio {
                                Some(selected_audio) if selected_audio == option => {
                                    true
                                },
                                _ => {
                                    false
                                },
                            }
                        })))
                        .event(clone!(state, option => move |_ :events::Change| {
                            actions::on_background_audio_click(Rc::clone(&state), elem.checked(), option);
                        }))
                    })
                }),
                html!("jig-audio-play-pause", {
                    .prop("slot", "play-pause")
                    .prop_signal("mode", audio_index.signal_ref(move |audio_index| {
                        match audio_index {
                            Some(audio_index) if *audio_index == index => "pause",
                            _ => "play",
                        }
                    }))
                    .event(clone!(option, audio_index, audio_handle => move |_ :events::Click| {
                        let on_ended = clone!(audio_index, audio_handle => move|| {
                            audio_index.set(None);
                            audio_handle.set(None);
                        });

                        let current_audio_index = audio_index.get();
                        match current_audio_index {
                            Some(current_audio_index) if current_audio_index == index => {
                                audio_index.set(None);
                                audio_handle.set(None);
                            },
                            _ => {
                                let path: AudioPath = option.clone().into();
                                let handle = AUDIO_MIXER.with(move |mixer| mixer.play_on_ended(path, false, on_ended));
                                audio_index.set(Some(index));
                                audio_handle.set(Some(handle));
                            }
                        }
                    }))
                }),
            ])
        })
    }
}
