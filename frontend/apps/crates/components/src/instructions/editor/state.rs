use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::jig::module::body::{Audio, Instructions};

use super::callbacks::Callbacks;

pub struct State {
    pub instructions: Mutable<Instructions>,
    pub callbacks: Callbacks,
}

impl State {
    pub fn new(instructions: Mutable<Instructions>, callbacks: Callbacks) -> Self {
        Self {
            instructions,
            callbacks,
        }
    }

    pub fn text_signal(&self) -> impl Signal<Item = String> {
        self.instructions
            .signal_cloned()
            .map(|instructions| match instructions.text {
                None => "".to_string(),
                Some(text) => text,
            })
    }
    pub fn audio_signal(&self) -> impl Signal<Item = Option<Audio>> {
        self.instructions
            .signal_cloned()
            .map(|instructions| instructions.audio)
    }
}
