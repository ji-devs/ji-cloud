use dominator::{Dom, clone, html};
use futures_signals::{map_ref, signal::SignalExt};
use std::rc::Rc;
use super::{
    components::{main_content, add_method, delete, main_action},
    state::*,
};
use utils::prelude::*;


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
                let add_method = state.add_method.signal_cloned()
                    => (*mode, *add_method)
            };

            sig.map(clone!(state => move |(mode, add_method)| {
                vec![
                    html!("empty-fragment", {
                        .property("slot", "main-action")
                        .child(main_action::render(state.clone(), mode, add_method))
                    }),
                    main_content::render(state.clone(), mode, add_method),
                ]
            }))
            .to_signal_vec()


        })
    })
}

fn get_element_mode(mode: AudioInputMode) -> String {
    match mode {
        AudioInputMode::Recording | AudioInputMode::Uploading => String::from("active"),
        AudioInputMode::Empty => String::from("default"),
        AudioInputMode::Stopped(_) => String::from("success"),
        AudioInputMode::Playing(_) => String::from("done"),
    }
}
