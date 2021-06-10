use shared::domain::jig::module::body::Audio;
use futures_signals::signal::{ReadOnlyMutable, Mutable, Signal, SignalExt};
use std::cell::RefCell;
use dominator_helpers::futures::AsyncLoader;
use dominator::clone;

pub struct AudioInputOptions<S: Signal<Item = Option<Audio>>> {
    pub ext_audio_signal: Option<S>
}


impl <S: Signal<Item = Option<Audio>>> AudioInputOptions <S> {

    pub fn new(ext_audio_signal: Option<S>) -> Self {
        Self {
            ext_audio_signal
        }
    }
}
