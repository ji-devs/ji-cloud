use std::rc::Rc;

use components::audio::mixer::{AudioHandle, AUDIO_MIXER};
use dominator::{clone, html, with_node, Dom};
use web_sys::HtmlInputElement;

use crate::edit::sidebar::jig::settings::{
    actions::{self, set_active_popup},
    dom::STR_BACK_TO_SETTINGS,
    state::ActiveSettingsPopup,
};
use futures_signals::signal::{Mutable, MutableLockMut, SignalExt};
use shared::domain::jig::AudioBackground;
use utils::{asset::JigAudioExt, events};

use super::super::state::State;

pub fn render(state: Rc<State>) -> Dom {
    let audio_handles: Vec<Mutable<Option<AudioHandle>>> = AudioBackground::variants()
        .iter()
        .map(|_| Mutable::new(None))
        .collect();
    let audio_handles = Rc::new(audio_handles);

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
        .children(AudioBackground::variants().iter().enumerate().map(clone!(state, audio_handles => move|(index, option)| {
            line(Rc::clone(&state), option, Rc::clone(&audio_handles), index)
        })).collect::<Vec<Dom>>())
        .after_removed(clone!(audio_handles => move |_| {
            for audio_handle in audio_handles.iter() {
                match &*audio_handle.lock_ref() {
                    None => {},
                    Some(audio_handle) => {
                        audio_handle.pause();
                    },
                }
            }
        }))
    })
}

fn line(
    state: Rc<State>,
    option: &AudioBackground,
    audio_handles: Rc<Vec<Mutable<Option<AudioHandle>>>>,
    index: usize,
) -> Dom {
    let audio_handle = &audio_handles[index];

    html!("jig-audio-line", {
        .prop("slot", "lines")
        .prop("label", option.display_name())
        .prop_signal("playing", audio_handle.signal_ref(|x| x.is_some()))
        .children(&mut [
            html!("input" => HtmlInputElement, {
                .with_node!(elem =>{
                    .prop("slot", "checkbox")
                    .prop("type", "checkbox")
                    .prop_signal("checked", state.background_audio.signal_cloned().map(clone!(option => move|selected_audio| {
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
                .prop_signal("mode", audio_handle.signal_ref(|audio_handle| {
                    match audio_handle {
                        Some(_) => "pause",
                        None => "play",
                    }
                }))
                .event(clone!(option, audio_handles => move |_ :events::Click| {
                    let on_ended = clone!(audio_handles => move|| {
                        audio_handles[index].set(None);
                    });

                    let mut audio_handles = audio_handles.iter().map(|x| x.lock_mut()).collect::<Vec<MutableLockMut<Option<AudioHandle>>>>();

                    match *audio_handles[index] {
                        Some(_) => *audio_handles[index] = None,
                        None => {
                            audio_handles = audio_handles.into_iter().map(|mut o| {
                                *o = None;
                                o
                            }).collect();

                            let handle = AUDIO_MIXER.with(move |mixer| mixer.play_on_ended(option.into(), false, on_ended));
                            *audio_handles[index] = Some(handle);
                        },
                    };
                }))
            }),
        ])
    })
}
