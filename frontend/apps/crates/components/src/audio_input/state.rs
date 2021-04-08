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
use super::options::*;

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

pub struct State {
    //on_change is called imperatively for every update
    //for example, to push to history
    pub on_change: Option<Box<dyn Fn(Option<AudioId>)>>,
    //audio_id is a mutable for affecting DOM
    //intermediate updates can be skipped
    pub audio_id: Mutable<Option<AudioId>>,
    pub mode: Mutable<AudioInputMode>,
    pub add_method: Mutable<AudioInputAddMethod>,
    pub recorder: AudioRecorder,
}

impl State {
    pub fn new(opts: AudioInputOptions) -> Self {
        let audio_id = Mutable::new(opts.audio_id); 

        Self {
            on_change: opts.on_change,
            audio_id,
            mode: Mutable::new(AudioInputMode::Record),
            recorder: AudioRecorder::new(),
            add_method: Mutable::new(AudioInputAddMethod::Record),
        }
    }

}

