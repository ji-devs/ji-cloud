use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use utils::events;

use crate::audio_input::state::{State, AudioInputMode, AudioInputAddMethod};



pub fn render(state: Rc<State>) -> Dom {
    html!("audio-input-delete", {
        .property("slot", "delete")
        .visible_signal(state.mode.signal_cloned().map(|mode| {
            match mode {
                AudioInputMode::Record | AudioInputMode::Recording | AudioInputMode::Upload | AudioInputMode::Uploading => false,
                AudioInputMode::Success | AudioInputMode::Playing => true,
            }
        }))
        .event(clone!(state => move |_: events::Click| {
            state.options.value.set(None);
            match state.add_method.lock_ref().clone() {
                AudioInputAddMethod::Record => state.mode.set(AudioInputMode::Record),
                AudioInputAddMethod::Upload => state.mode.set(AudioInputMode::Upload),
            };
        }))
    })
}