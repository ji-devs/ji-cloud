use std::rc::Rc;
use dominator::{Dom, clone, html};
use futures_signals::signal::Mutable;
use wasm_bindgen::{JsCast, prelude::Closure};
use shared::{domain::audio::AudioId, media::MediaLibrary};
use crate::audio_input::state::{State, AudioInputMode};
use web_sys::HtmlAudioElement;

pub fn on_time_update<F: Fn(Option<AudioId>) + 'static>(state:&State<F>, player: &HtmlAudioElement, current_time: &Mutable<i32>) {
    let time = time_to_percent(player.duration(), player.current_time());
    current_time.set(time as i32);
}

pub fn on_ended<F: Fn(Option<AudioId>) + 'static>(state:&State<F>) {
    state.mode.set(AudioInputMode::Success);
}

fn time_to_percent(total: f64, current: f64) -> f64 {
    (current / total) * 100_f64
}
