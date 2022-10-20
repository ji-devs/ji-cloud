use super::{AudioHandleId, AudioMessageFromTop, AudioMessageToTop, PlayAudioMessage, AUDIO_MIXER};
use itertools::Itertools;
use web_sys::{HtmlAudioElement, AudioContext, AudioContextState, Event};
use std::{cell::RefCell, collections::HashMap};
use utils::{prelude::*, js_wrappers::set_event_listener};

pub struct AudioMixerTop {
    audio_context: AudioContext,
    active: RefCell<HashMap<AudioHandleId, HtmlAudioElement>>,
    inactive: RefCell<Vec<HtmlAudioElement>>,
}

impl AudioMixerTop {
    pub(super) fn new() -> Self {
        let audio_context = create_audio_context();
        // let inactive = init_empty_audio_elements(10, &audio_context);
        // log::info!("inactive len {}", inactive.len());
        Self {
            audio_context,
            active: Default::default(),
            // inactive: RefCell::new(inactive),
            inactive: RefCell::new(vec![]),
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
                    // let _ = el.pause();
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

    fn play<F: FnMut() + 'static>(&self, audio_message: PlayAudioMessage, mut on_ended: F) {
        if self.inactive.borrow().len() == 0 { // TODO: dont like 
            let inactive = init_empty_audio_elements(10, &self.audio_context);
            *self.inactive.borrow_mut() = inactive;
        };


        // unwrapping, should never exceed number of items in pool
        let el = self.inactive.borrow_mut().pop().unwrap_ji();
        el.set_src(&audio_message.path);
        el.set_loop(audio_message.is_loop);
        // set_event_listener(&el, "ended", Box::new(move |e: Event| (on_ended)())); // TODO: need a way to get rid of these once removed, maybe have a central listener for all audio el that never get removed and call correct item

        // let _ = el.play();
        let res = el.play().unwrap_ji();
        web_sys::console::log_1(&res);
        log::info!("^ .play(), el ↓");
        web_sys::console::log_1(&el);
        log::info!("{}", &audio_message.path);
        self.active
            .borrow_mut()
            .insert(audio_message.handle_id, el);
    }

    fn handle_dropped(&self, handle_id: AudioHandleId) {
        let mut awsm_handles = self.active.borrow_mut();
        awsm_handles.remove(&handle_id);
    }

    fn pause_handle_called(&self, handle_id: AudioHandleId) {
        let awsm_handles = self.active.borrow();
        if let Some(audio) = awsm_handles.get(&handle_id) {
            // let _ = audio.pause();
        }
    }

    fn play_handle_called(&self, handle_id: AudioHandleId) {
        let awsm_handles = self.active.borrow();
        if let Some(audio) = awsm_handles.get(&handle_id) {
            let _ = audio.play();
        }
    }

    fn broadcast_context_available_request(&self) {
        let available = self.audio_context.state() == AudioContextState::Running;
        AUDIO_MIXER.with(|mixer| {
            mixer.set_context_available(available);
            mixer.message_all_iframes(AudioMessageFromTop::ContextAvailable(available));
        });
    }
}

fn init_empty_audio_elements(count: usize, context: &AudioContext) -> Vec<HtmlAudioElement> {
    log::info!("here {}", count);
    let mut i = 0usize;
    (0..count).map(|_| {
        log::info!("here");
        let el = create_audio_element_on_context(context);
        el.set_attribute("hay", &i.to_string());
        i += 1;
        el
    }).collect_vec()
}

// TODO: proper credit
const EMPTY_URL: &str = "data:audio/mpeg;base64,SUQzBAAAAAABEVRYWFgAAAAtAAADY29tbWVudABCaWdTb3VuZEJhbmsuY29tIC8gTGFTb25vdGhlcXVlLm9yZwBURU5DAAAAHQAAA1N3aXRjaCBQbHVzIMKpIE5DSCBTb2Z0d2FyZQBUSVQyAAAABgAAAzIyMzUAVFNTRQAAAA8AAANMYXZmNTcuODMuMTAwAAAAAAAAAAAAAAD/80DEAAAAA0gAAAAATEFNRTMuMTAwVVVVVVVVVVVVVUxBTUUzLjEwMFVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVf/zQsRbAAADSAAAAABVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVf/zQMSkAAADSAAAAABVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV";
fn create_audio_element_on_context(context: &AudioContext) -> HtmlAudioElement {
    let el = HtmlAudioElement::new()
        .unwrap_ji();
    // el.set_autoplay(true);
    el.set_src(EMPTY_URL);
    el.set_cross_origin(Some("anonymous"));
    let track = context.create_media_element_source(&el)
        .unwrap_ji();
    let _ = track.connect_with_audio_node(&context.destination());
    // let _ = el.play();
    // let _ = el.load();

    el.load();
    log::info!("after .load()");


    el
}

fn create_audio_context() -> AudioContext {
    AudioContext::new().unwrap_ji()
}
