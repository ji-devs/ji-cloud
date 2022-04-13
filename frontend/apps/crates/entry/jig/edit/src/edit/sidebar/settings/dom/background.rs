use std::rc::Rc;

use awsm_web::audio::{AudioClipOptions, AudioHandle};
use components::audio::mixer::{AudioSourceExt, AUDIO_MIXER};
use dominator::{clone, html, with_node, Dom};
use web_sys::HtmlInputElement;

use crate::edit::sidebar::settings::{
    actions::{self, set_active_popup},
    dom::STR_BACK_TO_SETTINGS,
    state::ActiveSettingsPopup,
};
use futures_signals::signal::{Mutable, MutableLockMut, SignalExt};
use shared::domain::jig::AudioBackground;
use utils::{events, jig::JigAudioExt};

use super::super::state::State;

pub fn render(state: Rc<State>) -> Dom {
    let audio_handles: Vec<Mutable<Option<AudioHandle>>> = AudioBackground::variants()
        .iter()
        .map(|_| Mutable::new(None))
        .collect();
    let audio_handles = Rc::new(audio_handles);

    html!("jig-audio-body", {
        .property("slot", "overlay")
        .property("kind", "background")
        .children(&mut [
            html!("button-rect", {
                .property("kind", "text")
                .property("slot", "back")
                .property("color", "blue")
                .child(html!("fa-icon", {.property("icon", "fa-light fa-chevron-left")}))
                .text(STR_BACK_TO_SETTINGS)
                .event(clone!(state => move|_: events::Click| {
                    set_active_popup(Rc::clone(&state), ActiveSettingsPopup::Main);
                }))
            }),
            html!("button-icon", {
                .property("icon", "x")
                .property("slot", "close")
                .event(clone!(state => move |_:events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            // html!("input-search", {
            //     .property("slot", "search")
            // }),
        ])
        .children(AudioBackground::variants().iter().enumerate().map(clone!(state, audio_handles => move|(index, option)| {
            line(Rc::clone(&state), option, Rc::clone(&audio_handles), index)
        })).collect::<Vec<Dom>>())
        .after_removed(clone!(audio_handles => move |_| {
            for audio_handle in audio_handles.iter() {
                match audio_handle.get_cloned() {
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
        .property("slot", "lines")
        .property("label", option.display_name())
        .property_signal("playing", audio_handle.signal_ref(|x| x.is_some()))
        .children(&mut [
            html!("input" => HtmlInputElement, {
                .with_node!(elem =>{
                    .property("slot", "checkbox")
                    .property("type", "checkbox")
                    .property_signal("checked", state.background_audio.signal_cloned().map(clone!(option => move|selected_audio| {
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
                .property("slot", "play-pause")
                .property_signal("mode", audio_handle.signal_ref(|audio_handle| {
                    match audio_handle {
                        Some(_) => "pause",
                        None => "play",
                    }
                }))
                .event(clone!(option, audio_handles => move |_ :events::Click| {
                    let on_ended = Some(clone!(audio_handles => move|| {
                        audio_handles[index].set(None);
                    }));

                    let mut audio_handles = audio_handles.iter().map(|x| x.lock_mut()).collect::<Vec<MutableLockMut<Option<AudioHandle>>>>();

                    match *audio_handles[index] {
                        Some(_) => *audio_handles[index] = None,
                        None => {
                            audio_handles = audio_handles.into_iter().map(|mut o| {
                                *o = None;
                                o
                            }).collect();

                            let handle = AUDIO_MIXER.with(|mixer| mixer.add_source(option.as_source(), AudioClipOptions {
                                auto_play: true,
                                is_loop: false,
                                on_ended,
                            }));
                            *audio_handles[index] = Some(handle);
                        },
                    };
                }))
            }),
        ])
    })
}
