use crate::audio_input::state::State;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Audio;
use std::rc::Rc;
use utils::{path::audio_lib_url, prelude::*};
use web_sys::HtmlAudioElement;

pub fn render(state: Rc<State>, audio: Audio) -> Dom {
    let current_time = Mutable::new(0);
    html!("progress-bar", {
        .property("slot", "main-content")
        .property("color", "green")
        .property_signal("progress", current_time.signal())
        .child(html!("audio" => HtmlAudioElement, {
            .property("autoplay", true)
            .property("src", audio_lib_url(audio.lib, audio.id))
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
