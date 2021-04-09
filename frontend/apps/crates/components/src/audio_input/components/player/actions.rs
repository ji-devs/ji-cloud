use futures_signals::signal::Mutable;
use shared::domain::audio::AudioId;
use crate::audio_input::state::{State, AudioInputMode};
use web_sys::HtmlAudioElement;

pub fn on_time_update(player: &HtmlAudioElement, current_time: &Mutable<i32>) {
    let time = time_to_percent(player.duration(), player.current_time());
    current_time.set(time as i32);
}

pub fn on_ended(state:&State, audio_id: AudioId) {
    state.mode.set(AudioInputMode::Stopped(audio_id));
}

fn time_to_percent(total: f64, current: f64) -> f64 {
    (current / total) * 100_f64
}
