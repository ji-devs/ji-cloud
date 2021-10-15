use crate::audio::mixer::AudioHandle;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Audio;
use std::rc::Rc;

pub struct AudioPlayerButton {
    pub audio: Audio,
    pub handle: Mutable<Option<AudioHandle>>,
}

impl AudioPlayerButton {
    pub fn new(audio: Audio) -> Rc<Self> {
        Rc::new(Self {
            audio,
            handle: Mutable::new(None),
        })
    }
}
