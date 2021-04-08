use dominator::{Dom, clone, html};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;
use super::{
    components::{main_content, add_method, delete, main_action},
    state::*,
    options::*,
};
use utils::prelude::*;
use shared::{domain::audio::AudioId, media::MediaLibrary};


pub fn render(state: Rc<State>, slot: Option<&str>) -> Dom {

    html!("audio-input", {
        .apply_if(slot.is_some(), move |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
        .property_signal("mode", state.mode.signal_cloned().map(|mode| get_element_mode(mode)))
        .children(&mut [
            add_method::render(state.clone(), AudioInputAddMethod::Record),
            add_method::render(state.clone(), AudioInputAddMethod::Upload),
            delete::render(state.clone()),
        ])
        .children_signal_vec({
            let sig = map_ref! {
                let mode = state.mode.signal_cloned(),
                let add_method = state.add_method.signal_cloned(),
                let audio_id = state.audio_id.signal_cloned()
                    => (*mode, *add_method, *audio_id)
            };

            sig.map(clone!(state => move |(mode, add_method, audio_id)| {
                vec![
                    html!("empty-fragment", {
                        .property("slot", "main-action")
                        .child(main_action::render(state.clone(), mode, add_method, audio_id))
                    }),
                    main_content::render(state.clone(), mode, add_method, audio_id),
                ]
            }))
            .to_signal_vec()


        })
    })
}

fn get_element_mode(mode: AudioInputMode) -> String {
    match mode {
        AudioInputMode::Record => String::from("default"),
        AudioInputMode::Recording => String::from("active"),
        AudioInputMode::Upload => String::from("default"),
        AudioInputMode::Uploading => String::from("active"),
        AudioInputMode::Success => String::from("success"),
        AudioInputMode::Playing => String::from("done"),
    }
}
