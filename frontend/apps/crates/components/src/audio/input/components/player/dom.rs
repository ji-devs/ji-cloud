use crate::audio::input::state::AudioInput;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::Mutable;
use shared::domain::module::body::Audio;
use std::rc::Rc;
use utils::{path::audio_lib_url, prelude::*};
use web_sys::HtmlAudioElement;

pub fn render(state: Rc<AudioInput>, audio: Audio) -> Dom {
    let current_time = Mutable::new(0);
    html!("progress-bar", {
        .prop("slot", "main-content")
        .prop("color", "green")
        .prop_signal("progress", current_time.signal())
        .child(html!("audio" => HtmlAudioElement, {
            .prop("autoplay", true)
            .prop("src", audio_lib_url(audio.lib, audio.id))
            .with_node!(elem => {
                .event(clone!(current_time => move |_:events::TimeUpdate| {
                    super::actions::on_time_update(&elem, &current_time);
                }))
            })
            .event(clone!(state, audio => move |_:events::Ended| {
                super::actions::on_ended(&state, audio.clone());
            }))
        }))
    })
}
