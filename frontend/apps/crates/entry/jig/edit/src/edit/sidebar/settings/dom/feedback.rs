use core::hash::Hash;
use std::{collections::HashSet, rc::Rc};

use awsm_web::audio::AudioClipOptions;
use components::audio::mixer::{AudioHandle, AudioPath, AUDIO_MIXER};
use dominator::{clone, html, Dom};

use crate::edit::sidebar::settings::{
    actions::{set_active_popup, update_jig_settings},
    dom::STR_BACK_TO_SETTINGS,
    state::{ActiveSettingsPopup, FeedbackTab},
};
use futures_signals::signal::{Mutable, MutableLockMut, SignalExt};
use shared::domain::jig::{AudioFeedbackNegative, AudioFeedbackPositive};
use utils::{events, jig::JigAudioExt};

use super::super::state::State;

const STR_CORRECT: &'static str = "Correct answer";
const STR_MISTAKE: &'static str = "Mistake";

pub fn render(state: Rc<State>, tab: FeedbackTab) -> Dom {
    html!("jig-audio-body", {
        .property("slot", "overlay")
        .property("kind", "feedback")
        .children(&mut [
            html!("label", {
                .property("slot", "correct-mistake")
                .child(html!("input", {
                    .property("name", "correct-mistake")
                    .property("type", "radio")
                    .property("checked", tab == FeedbackTab::Positive)
                    .event(clone!(state => move |_:events::Input| {
                        state.active_popup.set(Some(ActiveSettingsPopup::Feedback(FeedbackTab::Positive)));
                    }))
                }))
                .text(STR_CORRECT)
            }),
            html!("label", {
                .property("slot", "correct-mistake")
                .child(html!("input", {
                    .property("name", "correct-mistake")
                    .property("type", "radio")
                    .property("checked", tab == FeedbackTab::Negative)
                    .event(clone!(state => move |_:events::Input| {
                        state.active_popup.set(Some(ActiveSettingsPopup::Feedback(FeedbackTab::Negative)));
                    }))
                }))
                .text(STR_MISTAKE)
            }),
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
        .apply(|dom| {
            match tab {
                FeedbackTab::Positive => {
                    let audio_handles: Vec<Mutable<Option<AudioHandle>>> = AudioFeedbackPositive::variants().iter().map(|_| Mutable::new(None)).collect();
                    let audio_handles = Rc::new(audio_handles);

                    dom.children(AudioFeedbackPositive::variants().iter().enumerate().map(clone!(state => move|(index, option)| {
                        line(Rc::clone(&state), state.feedback_positive.clone(), option, audio_handles.clone(), index)
                    })).collect::<Vec<Dom>>())
                },
                FeedbackTab::Negative => {
                    let audio_handles: Vec<Mutable<Option<AudioHandle>>> = AudioFeedbackNegative::variants().iter().map(|_| Mutable::new(None)).collect();
                    let audio_handles = Rc::new(audio_handles);

                    dom.children(AudioFeedbackNegative::variants().iter().enumerate().map(clone!(state => move|(index, option)| {
                        line(Rc::clone(&state), state.feedback_negative.clone(), option, audio_handles.clone(), index)
                    })).collect::<Vec<Dom>>())
                },
            }
        })
    })
}

fn line<'a, T>(
    state: Rc<State>,
    list: Mutable<HashSet<T>>,
    option: &T,
    audio_handles: Rc<Vec<Mutable<Option<AudioHandle>>>>,
    index: usize,
) -> Dom
where
    T: Hash + Eq + Clone + JigAudioExt + Into<AudioPath<'a>> + std::fmt::Debug + 'static,
{
    let audio_handle = &audio_handles[index];

    html!("jig-audio-line", {
        .property("slot", "lines")
        .property("label", option.display_name())
        .property_signal("playing", audio_handle.signal_ref(|x| x.is_some()))
        .children(&mut [
            html!("input", {
                .property("slot", "checkbox")
                .property("type", "checkbox")
                .property_signal("checked", list.signal_cloned().map(clone!(option => move|list| {
                    list.contains(&option)
                })))
                .event(clone!(state, option => move |_ :events::Click| {
                    let mut list = list.lock_mut();
                    match list.contains(&option) {
                        true => list.remove(&option),
                        false => list.insert(option.clone()),
                    };
                    drop(list);
                    update_jig_settings(Rc::clone(&state));
                }))
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

                            let path:AudioPath = option.clone().into();

                            let handle = AUDIO_MIXER.with(move |mixer| mixer.add_source(path, AudioClipOptions {
                                auto_play: true,
                                is_loop: false,
                                on_ended: on_ended,
                            }));
                            *audio_handles[index] = Some(handle);
                        },
                    };
                }))
            }),
        ])
    })
}
