use std::rc::Rc;
use dominator::{Dom, clone, html};
use utils::prelude::*;
use wasm_bindgen_futures::spawn_local;
use shared::{domain::audio::AudioId, media::MediaLibrary};
use crate::audio_input::state::{State, AudioInputMode, AudioInputAddMethod};
use super::super::actions::file_change;



pub fn render<F: Fn(Option<AudioId>) + 'static>(state: Rc<State<F>>, mode:AudioInputMode, add_method: AudioInputAddMethod, audio_id: Option<AudioId>) -> Dom {
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
                //TODO - clean all this up!
                if let Some(audio_id) = audio_id {
                    match mode {
                        AudioInputMode::Record => String::from("record"),
                        AudioInputMode::Recording => String::from("stop"),
                        AudioInputMode::Upload => String::from("add-sound"),
                        AudioInputMode::Success => String::from("play"),
                        AudioInputMode::Playing => String::from("stop"),
                        AudioInputMode::Uploading => panic!("Imposible"),
                    }
                } else {
                    match mode {
                        AudioInputMode::Record => String::from("record"),
                        AudioInputMode::Recording => String::from("stop"),
                        AudioInputMode::Upload => String::from("add-sound"),
                        AudioInputMode::Uploading => panic!("Imposible"),

                        AudioInputMode::Success | AudioInputMode::Playing => {
                            match add_method {
                                AudioInputAddMethod::Record => String::from("record"),
                                _ => unimplemented!("TODO: success or play in upload mode...")
                            }
                        }
                    }
                }
            })
            .event(clone!(state => move |_: events::Click| {
                match mode {
                    AudioInputMode::Record => {
                        spawn_local(clone!(state => async move {
                            state.recorder.start().await;
                            state.mode.set_neq(AudioInputMode::Recording);
                        }));
                    },
                    AudioInputMode::Recording => {
                        spawn_local(clone!(state => async move {
                            let file = state.recorder.stop().await;
                            file_change(state.clone(), file).await;
                        }));
                    },
                    AudioInputMode::Upload => {
                        state.mode.set_neq(AudioInputMode::Record);
                        state.add_method.set(AudioInputAddMethod::Record);
                    },
                    AudioInputMode::Success => {
                        state.mode.set(AudioInputMode::Playing);
                    },
                    AudioInputMode::Playing => {
                        state.mode.set_neq(AudioInputMode::Success);
                    },
                    AudioInputMode::Uploading => panic!("Should not be posible"),
                };
            }))
        })
    }
}
