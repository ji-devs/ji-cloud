use std::rc::Rc;
use utils::path::audio_lib_url;
use wasm_bindgen::{JsCast, prelude::*};
use dominator::clone;
use futures::future::ready;
use futures_signals::signal::{Mutable, SignalExt};
use shared::{domain::audio::AudioId, media::MediaLibrary};
use web_sys::HtmlAudioElement;
use wasm_bindgen_futures::spawn_local;
use super::recorder::AudioRecorder;


#[derive(Clone)]
pub enum AudioInputMode {
    Record,
    Recording,
    Upload,
    Uploading,
    Success,
    Playing,
}

#[derive(Clone, PartialEq)]
pub enum AudioInputAddMethod {
    Record,
    Upload,
}

pub struct State {
    pub options: AudioInputOptions,
    pub mode: Mutable<AudioInputMode>,
    pub add_method: Mutable<AudioInputAddMethod>,
    pub player: HtmlAudioElement,
    pub recorder: AudioRecorder,
}

impl State {
    pub fn new(options: AudioInputOptions) -> Rc<Self> {
        let state = Rc::new(Self {
            options,
            mode: Mutable::new(AudioInputMode::Record),
            player: HtmlAudioElement::new().unwrap(),
            recorder: AudioRecorder::new(),
            add_method: Mutable::new(AudioInputAddMethod::Record),
        });

        Self::wire_up_player(state.clone());

        state
    }

    fn wire_up_player(state: Rc<State>) {
        let closure = Closure::wrap(Box::new(clone!(state => move |_: web_sys::Event| {
            state.mode.set(AudioInputMode::Success);
        })) as Box<dyn FnMut(_)>);
        state.player.add_event_listener_with_callback("ended", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
        spawn_local(state.options.value.signal_cloned().for_each(clone!(state => move |audio_id| {
            match audio_id {
                Some(audio_id) => state.player.set_src(&audio_lib_url(MediaLibrary::User, audio_id)),
                None => state.player.set_src(""),
            };
            ready(())
        })));
    }
}

pub struct AudioInputOptions {
    pub value: Mutable<Option<AudioId>>
}
