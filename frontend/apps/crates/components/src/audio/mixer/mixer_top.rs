use super::{AudioHandleId, AudioMessageFromTop, AudioMessageToTop, PlayAudioMessage, AUDIO_MIXER};
use dominator::clone;
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};
use utils::{js_wrappers::set_event_listener, prelude::*};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{AudioContext, Event, HtmlAudioElement};

const EMPTY_AUDIO_URL: &str = "data:audio/mpeg;base64,//uQxAAAAAAAAAAAAAAAAAAAAAAASW5mbwAAAA8AAAABAAADQgD///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////8AAAA6TEFNRTMuMTAwAc0AAAAAAAAAABSAJAJAQgAAgAAAA0LqRHv+AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA//uQxAADwAABpAAAACAAADSAAAAETEFNRTMuMTAwVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV";

thread_local! {
    // Using a vec because HtmlAudioElement does not implement Hash.
    static ENDED_CALLBACKS: RefCell<Vec<EndedCallback>> = Default::default();
}

struct EndedCallback {
    el: HtmlAudioElement,
    handle_id: AudioHandleId,
    settle_on_error: bool,
    callback: Box<dyn FnMut()>,
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
            AudioMessageToTop::InitSilently => {
                self.init_if_not_ready();
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
            AudioMessageToTop::PauseAll { except } => {
                for (handle, el) in self.active.borrow().iter() {
                    if !except.contains(handle) {
                        let _ = el.pause();
                    }
                }
            }
            AudioMessageToTop::PlayAll { except } => {
                for (handle, el) in self.active.borrow().iter() {
                    if !except.contains(handle) && !el.ended() {
                        play_audio_element(el, handle);
                    }
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
            ended_callbacks.borrow_mut().push(EndedCallback {
                el: el.clone(),
                handle_id: audio_message.handle_id.clone(),
                settle_on_error: audio_message.settle_on_error,
                callback: Box::new(on_ended),
            });
        });

        self.active
            .borrow_mut()
            .insert(audio_message.handle_id.clone(), el.clone());
        play_audio_element(&el, &audio_message.handle_id);
    }

    fn handle_dropped(&self, handle_id: AudioHandleId) {
        let el = self.active.borrow_mut().remove(&handle_id);
        if let Some(el) = el {
            let callback = ENDED_CALLBACKS.with(|ended_callbacks| {
                let mut ended_callbacks = ended_callbacks.borrow_mut();
                ended_callbacks
                    .iter()
                    .position(|entry| entry.handle_id == handle_id)
                    .map(|index| ended_callbacks.remove(index))
            });
            drop(callback);

            el.set_src(EMPTY_AUDIO_URL);
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
            play_audio_element(audio, &handle_id);
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
            if el.ended() {
                run_ended_callback(|entry| entry.el == el);
            }
        })),
    );
    set_event_listener(
        &el,
        "error",
        Box::new(clone!(el => move |_: Event| {
            if el.error().is_some() {
                run_ended_callback(|entry| entry.el == el && entry.settle_on_error);
            }
        })),
    );
    el
}

fn run_ended_callback(matches: impl Fn(&EndedCallback) -> bool) {
    let callback = ENDED_CALLBACKS.with(|ended_callbacks| {
        let mut ended_callbacks = ended_callbacks.borrow_mut();
        ended_callbacks
            .iter()
            .position(matches)
            .map(|index| ended_callbacks.remove(index))
    });

    if let Some(mut entry) = callback {
        (entry.callback)();
    }
}

fn play_audio_element(el: &HtmlAudioElement, handle_id: &AudioHandleId) {
    let result = el.play();
    let settle_on_error = ENDED_CALLBACKS.with(|ended_callbacks| {
        ended_callbacks
            .borrow()
            .iter()
            .any(|entry| entry.handle_id == *handle_id && entry.settle_on_error)
    });

    if settle_on_error {
        spawn_local(clone!(handle_id => async move {
            let result = match result {
                Ok(promise) => JsFuture::from(promise).await,
                Err(error) => Err(error),
            };

            if let Err(error) = result {
                // Pausing while play is pending aborts the promise, but playback can resume.
                let name = js_sys::Reflect::get(&error, &"name".into()).ok();
                if name.and_then(|name| name.as_string()).as_deref() != Some("AbortError") {
                    run_ended_callback(|entry| entry.handle_id == handle_id);
                }
            }
        }));
    }
}

fn create_audio_context() -> AudioContext {
    AudioContext::new().unwrap_ji()
}
