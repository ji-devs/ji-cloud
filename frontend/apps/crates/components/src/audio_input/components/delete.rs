use std::rc::Rc;

use dominator::{Dom, clone, html};
use utils::events;
use crate::audio_input::state::{State, AudioInputMode};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::jig::module::body::Audio;

pub fn render (state: Rc<State>) -> Dom {
    html!("audio-input-delete", {
        .property("slot", "delete")
        .visible_signal(state.mode.signal_cloned().map(|mode| {
            match mode {
                AudioInputMode::Empty | AudioInputMode::Recording | AudioInputMode::Uploading => false,
                AudioInputMode::Stopped(_) | AudioInputMode::Playing(_) => true,
            }
        }))
        .event(clone!(state => move |_: events::Click| {
            state.set_audio(None);
        }))
    })
}
