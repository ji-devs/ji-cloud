use std::{rc::Rc, cell::RefCell};
use utils::{prelude::*, path::audio_lib_url};
use wasm_bindgen::{JsCast, prelude::*};
use dominator::clone;
use futures::future::ready;
use futures_signals::signal::{Mutable, SignalExt};
use shared::{domain::audio::AudioId, media::MediaLibrary};
use web_sys::HtmlAudioElement;
use wasm_bindgen_futures::spawn_local;
use super::recorder::AudioRecorder;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioInputMode {
    Record,
    Recording,
    Upload,
    Uploading,
    Success,
    Playing,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioInputAddMethod {
    Record,
    Upload,
}

pub struct State <F: Fn(Option<AudioId>)> {
    //on_change is called imperatively for every update
    //for example, to push to history
    pub on_change: Option<F>,
    //audio_id is a mutable for affecting DOM
    //intermediate updates can be skipped
    pub audio_id: Mutable<Option<AudioId>>,
    pub mode: Mutable<AudioInputMode>,
    pub add_method: Mutable<AudioInputAddMethod>,
    pub recorder: AudioRecorder,
}

impl <F: Fn(Option<AudioId>) + 'static> State <F> {
    pub fn new(options: AudioInputOptions<F>) -> Self {
        let audio_id = options.audio_id.unwrap_or(Mutable::new(None));

        Self {
            on_change: options.on_change,
            audio_id,
            mode: Mutable::new(AudioInputMode::Record),
            recorder: AudioRecorder::new(),
            add_method: Mutable::new(AudioInputAddMethod::Record),
        }
    }

}

pub struct AudioInputOptions <F: Fn(Option<AudioId>)> {
    pub on_change: Option<F>,
    pub audio_id: Option<Mutable<Option<AudioId>>>,
}
