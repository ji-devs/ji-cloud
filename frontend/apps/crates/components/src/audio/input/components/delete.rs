use std::rc::Rc;

use crate::audio::input::state::{AudioInput, AudioInputMode};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

pub fn render(state: Rc<AudioInput>) -> Dom {
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
