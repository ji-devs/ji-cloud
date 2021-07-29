use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use shared::{
    domain::{
        jig::module::body::{Audio, Instructions},
        audio::AudioId,
    },
    media::MediaLibrary
};
use crate::audio_input::{
    dom::render as render_audio_input,
    options::AudioInputOptions,
    state::State as AudioState,
    callbacks::Callbacks as AudioCallbacks,
};
use futures::stream::StreamExt;
use super::callbacks::Callbacks;


pub struct State {
    pub instructions: Mutable<Instructions>,
    pub callbacks: Callbacks
}

impl State {
    pub fn new(instructions: Mutable<Instructions>, callbacks: Callbacks) -> Self {
        Self {
            instructions,
            callbacks
        }
    }


    pub fn text_signal(&self) -> impl Signal<Item = String> {
        self.instructions.signal_cloned().map(|instructions| {
            match instructions.text {
                None => "".to_string(),
                Some(text) => text
            }
        })
    }
    pub fn audio_signal(&self) -> impl Signal<Item = Option<Audio>> {
        self.instructions.signal_cloned().map(|instructions| {
            instructions.audio
        })
    }

}
