use std::rc::Rc;
use dominator::{Dom, clone, html};
use utils::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::audio_input::state::{State, AudioInputMode, AudioInputAddMethod};
use super::super::actions::file_change;


pub fn render(state: Rc<State>, mode: AudioInputMode, add_method: AudioInputAddMethod) -> Dom {
    if let AudioInputMode::Uploading = mode {
        html!("button-text", {
            .property("slot", "main-action")
            .text("Cancel")
            .event(clone!(state => move |_: events::Click| {
                todo!();
            }))
        })
    } else {
        html!("audio-input-action", {
            .property("slot", "main-action")
            .property("kind", {

                match mode {
                    AudioInputMode::Empty => {
                        match add_method {
                            AudioInputAddMethod::Record => String::from("record"),
                            AudioInputAddMethod::Upload => String::from("add-sound"),
                        }
                    },
                    AudioInputMode::Recording => String::from("stop"),
                    AudioInputMode::Stopped(_) => String::from("play"),
                    AudioInputMode::Playing(_) => String::from("stop"),
                    AudioInputMode::Uploading => panic!("Imposible"),
                }
            })
            .event(clone!(state, mode => move |_: events::Click| {
                match mode.clone() {
                    AudioInputMode::Empty => {
                        match add_method {
                            AudioInputAddMethod::Record => {
                                spawn_local(clone!(state => async move {
                                    state.recorder.start().await;
                                    state.mode.set_neq(AudioInputMode::Recording);
                                }));
                            }
                            AudioInputAddMethod::Upload => {
                                state.mode.set_neq(AudioInputMode::Empty);
                                state.add_method.set(AudioInputAddMethod::Record);
                            }
                        }
                    },
                    AudioInputMode::Recording => {
                        spawn_local(clone!(state => async move {
                            let file = state.recorder.stop().await;
                            file_change(state.clone(), file).await;
                        }));
                    },
                    AudioInputMode::Stopped(audio) => {
                        state.mode.set(AudioInputMode::Playing(audio));
                    },
                    AudioInputMode::Playing(audio) => {
                        state.mode.set_neq(AudioInputMode::Stopped(audio));
                    },
                    AudioInputMode::Uploading => panic!("Should not be posible"),
                };
            }))
        })
    }
}
