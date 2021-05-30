use futures_signals::signal::{Mutable};
use shared::domain::jig::module::body::Audio;
use super::recorder::AudioRecorder;
use super::options::*;
use super::callbacks::Callbacks;

#[derive(Clone, Debug, PartialEq)]
pub enum AudioInputMode {
    Playing(Audio),
    Stopped(Audio),
    Empty,
    Recording,
    Uploading,
    // Paused(AudioId, Timecode) we don't have a design for this but might be useful
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioInputAddMethod {
    Record,
    Upload,
}

pub struct State {
    pub callbacks: Callbacks,
    //audio_id is a mutable for affecting DOM
    //intermediate updates can be skipped
    pub mode: Mutable<AudioInputMode>,
    pub add_method: Mutable<AudioInputAddMethod>,
    pub recorder: AudioRecorder,
}

impl State {
    pub fn new(opts: AudioInputOptions, callbacks: Callbacks) -> Self {
        let mode = match opts.audio {
            Some(audio) => AudioInputMode::Stopped(audio),
            None => AudioInputMode::Empty,
        };

        Self {
            callbacks, 
            mode: Mutable::new(mode),
            recorder: AudioRecorder::new(),
            add_method: Mutable::new(AudioInputAddMethod::Record),
        }
    }
}
