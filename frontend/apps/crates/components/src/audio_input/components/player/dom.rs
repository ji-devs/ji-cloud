use std::rc::Rc;
use utils::{path::audio_lib_url, prelude::*};
use dominator::{Dom, clone, html, with_node};
use futures_signals::signal::Mutable;
use shared::{domain::audio::AudioId, media::MediaLibrary};
use crate::audio_input::state::State;
use web_sys::HtmlAudioElement;

pub fn render(state: Rc<State>, audio_id: AudioId) -> Dom {
    let current_time = Mutable::new(0);
    html!("progress-bar", {
        .property("slot", "main-content")
        .property("color", "green")
        .property_signal("progress", current_time.signal())
        .child(html!("audio" => HtmlAudioElement, {
            .property("autoplay", true)
            .property("src", audio_lib_url(MediaLibrary::User, audio_id))
            .with_node!(elem => { 
                .event(clone!(current_time => move |_:events::TimeUpdate| {
                    super::actions::on_time_update(&elem, &current_time);
                }))
            })
            .event(clone!(state, audio_id => move |_:events::Ended| {
                super::actions::on_ended(&state, audio_id);
            }))
        }))
    })
}
