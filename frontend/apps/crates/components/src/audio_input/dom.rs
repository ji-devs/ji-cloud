use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use super::{
    components::{main_content, add_method, delete, main_action},
    state::{AudioInputAddMethod, AudioInputMode, AudioInputOptions, State}
};


pub fn render(audio_input_options: AudioInputOptions) -> Dom {
    let state: Rc<State> = State::new(audio_input_options);

    html!("audio-input", {
        .property_signal("mode", state.mode.signal_cloned().map(|mode| get_element_mode(mode)))
        .children(&mut [
            add_method::render(state.clone(), AudioInputAddMethod::Record),
            add_method::render(state.clone(), AudioInputAddMethod::Upload),
            delete::render(state.clone()),
        ])
        .child(html!("empty-fragment", {
            .property("slot", "main-action")
            .child_signal(state.mode.signal_cloned().map(clone!(state => move |_| Some(main_action::render(state.clone())))))
        }))
        .child_signal(state.mode.signal_cloned().map(clone!(state => move |_| Some(main_content::render(state.clone())))))
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
