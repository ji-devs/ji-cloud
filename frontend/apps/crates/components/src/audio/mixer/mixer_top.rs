use super::{AudioHandleId, AudioMessageFromTop, AudioMessageToTop, PlayAudioMessage, AUDIO_MIXER};
use dominator::clone;
use gloo_timers::future::TimeoutFuture;
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};
use utils::{js_wrappers::set_event_listener, prelude::*};
use wasm_bindgen_futures::spawn_local;
use web_sys::{AudioContext, Event, HtmlAudioElement};

const EMPTY_AUDIO_URL: &str = "data:audio/mpeg;base64,//uQxAAAAAAAAAAAAAAAAAAAAAAASW5mbwAAAA8AAAABAAADQgD///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////8AAAA6TEFNRTMuMTAwAc0AAAAAAAAAABSAJAJAQgAAgAAAA0LqRHv+AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA//uQxAADwAABpAAAACAAADSAAAAETEFNRTMuMTAwVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV";

thread_local! {
    // using vec of tuples instead of hashmap because HtmlAudioElement doesn't implement Hash
    static ENDED_CALLBACKS: RefCell<Vec<(HtmlAudioElement, Box<dyn FnMut()>)>> = Default::default();
}

pub struct AudioMixerTop {
    // if none than context is not ready yet
    audio_context: RefCell<Option<AudioContext>>,
    active: RefCell<HashMap<AudioHandleId, HtmlAudioElement>>,
    inactive: RefCell<Vec<HtmlAudioElement>>,
}

impl AudioMixerTop {
    pub(super) fn new() -> Self {
        Self {
            audio_context: Default::default(),
            active: Default::default(),
            inactive: Default::default(),
        }
    }

    pub(super) fn run_audio_message(&self, message: AudioMessageToTop) {
        match message {
            AudioMessageToTop::Play(play_message) => {
                let handle_id = play_message.handle_id.clone();
                self.play(play_message, move || {
                    AUDIO_MIXER.with(|mixer| mixer.done_playing(handle_id.clone()))
                });
            }
            AudioMessageToTop::PauseHandleCalled(handle_id) => {
                self.pause_handle_called(handle_id);
            }
            AudioMessageToTop::PlayHandleCalled(handle_id) => {
                self.play_handle_called(handle_id);
            }
            AudioMessageToTop::HandleDropped(handle_id) => {
                self.handle_dropped(handle_id);
            }
            AudioMessageToTop::PauseAll => {
                for (_, el) in self.active.borrow().iter() {
                    let _ = el.pause();
                }
            }
            AudioMessageToTop::PlayAll => {
                for (_, el) in self.active.borrow().iter() {
                    let _ = el.play();
                }
            }
            AudioMessageToTop::BroadcastContextAvailable => {
                self.broadcast_context_available_request();
            }
        }
    }

    fn init_if_not_ready(&self) {
        if self.audio_context.borrow().is_none() {
            let audio_context = create_audio_context();
            *self.inactive.borrow_mut() = init_empty_audio_elements(10, &audio_context);
            *self.audio_context.borrow_mut() = Some(audio_context);
        }
    }

    fn play<F: FnMut() + 'static>(&self, audio_message: PlayAudioMessage, on_ended: F) {
        self.init_if_not_ready();

        // Unwrapping. Should never exceed number of items in pool
        let el = self.inactive.borrow_mut().pop().unwrap_ji();
        el.set_src(&audio_message.url);
        el.set_loop(audio_message.is_loop);
        ENDED_CALLBACKS.with(|ended_callbacks| {
            ended_callbacks
                .borrow_mut()
                .push((el.clone(), Box::new(on_ended)));
        });

        let _ = el.play();
        self.active.borrow_mut().insert(audio_message.handle_id, el);
    }

    fn handle_dropped(&self, handle_id: AudioHandleId) {
        let mut active = self.active.borrow_mut();
        let el = active.remove(&handle_id);
        if let Some(el) = &el {
            spawn_local(clone!(el => async move {
                // wait for next cycle as ended_callbacks is currently locked because handle_dropped is called from within a callback
                TimeoutFuture::new(0).await;
                ENDED_CALLBACKS.with(clone!(el => move |ended_callbacks| {
                    let mut ended_callbacks = ended_callbacks.borrow_mut();
                    if let Some(index) = ended_callbacks.iter().position(|(el2, _)| el2 == &el) {
                        let _ = ended_callbacks.remove(index);
                    }
                }));
            }));
        }
        if let Some(el) = el {
            self.inactive.borrow_mut().push(el);
        }
    }

    fn pause_handle_called(&self, handle_id: AudioHandleId) {
        let active = self.active.borrow();
        if let Some(audio) = active.get(&handle_id) {
            let _ = audio.pause();
        }
    }

    fn play_handle_called(&self, handle_id: AudioHandleId) {
        let active = self.active.borrow();
        if let Some(audio) = active.get(&handle_id) {
            let _ = audio.play();
        }
    }

    fn broadcast_context_available_request(&self) {
        let available = self.audio_context.borrow().is_some();
        AUDIO_MIXER.with(|mixer| {
            mixer.set_context_available(available);
            mixer.message_all_iframes(AudioMessageFromTop::ContextAvailable(available));
        });
    }
}

fn init_empty_audio_elements(count: usize, context: &AudioContext) -> Vec<HtmlAudioElement> {
    (0..count)
        .map(|_| create_audio_element_on_context(context))
        .collect_vec()
}

fn create_audio_element_on_context(context: &AudioContext) -> HtmlAudioElement {
    let el = HtmlAudioElement::new().unwrap_ji();
    el.set_src(EMPTY_AUDIO_URL);
    el.set_cross_origin(Some("anonymous"));
    let track = context.create_media_element_source(&el).unwrap_ji();
    let _ = track.connect_with_audio_node(&context.destination());
    el.load();
    set_event_listener(
        &el,
        "ended",
        Box::new(clone!(el => move |_: Event| {
            ENDED_CALLBACKS.with(clone!(el => move |ended_callbacks| {
                let mut ended_callbacks = ended_callbacks.borrow_mut();
                let callback = ended_callbacks.iter_mut().find_map(|(el2, callback)| {
                    if el2 == &el {
                        Some(callback)
                    } else {
                        None
                    }
                });
                if let Some(callback) = callback {
                    (callback)();
                }
            }));
        })),
    );
    el
}

fn create_audio_context() -> AudioContext {
    AudioContext::new().unwrap_ji()
}
