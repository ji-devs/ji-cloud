use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use utils::events;
use crate::audio_input::state::{State, AudioInputMode};


pub fn render(state: Rc<State>) -> Dom {
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
