use std::rc::Rc;

use dominator::{Dom, clone, html};
use utils::events;
use wasm_bindgen_futures::spawn_local;

use crate::audio_input::state::{State, AudioInputMode, AudioInputAddMethod};
use super::super::actions::file_change;



pub fn render(state: Rc<State>) -> Dom {
    let mode = state.mode.lock_ref();
    if let AudioInputMode::Uploading = *mode {
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
                match *mode {
                    AudioInputMode::Record => String::from("record"),
                    AudioInputMode::Recording => String::from("stop"),
                    AudioInputMode::Upload => String::from("add-sound"),
                    AudioInputMode::Success => String::from("play"),
                    AudioInputMode::Playing => String::from("stop"),
                    AudioInputMode::Uploading => panic!("Imposible"),
                }
            })
            .event(clone!(state => move |_: events::Click| {
                let mut mode = state.mode.lock_mut();
                match *mode {
                    AudioInputMode::Record => {
                        spawn_local(clone!(state => async move {
                            state.recorder.start().await;
                            state.mode.set(AudioInputMode::Recording);
                        }));
                    },
                    AudioInputMode::Recording => {
                        spawn_local(clone!(state => async move {
                            let file = state.recorder.stop().await;
                            file_change(state.clone(), file).await;
                        }));
                    },
                    AudioInputMode::Upload => {
                        *mode = AudioInputMode::Record;
                        state.add_method.set(AudioInputAddMethod::Record);
                    },
                    AudioInputMode::Success => {
                        spawn_local(clone!(state => async move {
                            let play_promise = state.player.play().unwrap();
                            let _ = wasm_bindgen_futures::JsFuture::from(play_promise).await;
                            state.mode.set(AudioInputMode::Playing);
                        }));
                    },
                    AudioInputMode::Playing => {
                        spawn_local(clone!(state => async move {
                            let _ = state.player.pause().unwrap();
                            state.mode.set(AudioInputMode::Success);
                        }));
                    },
                    AudioInputMode::Uploading => panic!("Should not be posible"),
                };
            }))
        })
    }
}
