use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use shared::{
    domain::{
        jig::module::body::{Audio, Instructions},
        audio::AudioId,
    },
    media::MediaLibrary
};
use crate::audio_input::{
    dom::render as render_audio_input,
    options::AudioInputOptions,
    state::State as AudioState,
    callbacks::Callbacks as AudioCallbacks,
};
use futures::stream::StreamExt;
use super::state::State;

pub const STR_INSTRUCTIONS_LABEL:&'static str = "Written instructions";
pub const STR_INSTRUCTIONS_PLACEHOLDER:&'static str = "Type instructions";

pub fn render(state:Rc<State>) -> Dom {
    html!("div", {
        .children(&mut [
            render_text(state.clone()),
            render_audio(state.clone())
        ])
    })
}


pub fn render_text(state: Rc<State>) -> Dom {
    fn change_text(state: &State, text: String, push_history:bool) {
        let mut lock = state.instructions.lock_mut();

        if text == "" {
            lock.text = None;
        } else {
            lock.text = Some(text);
        }
        (state.callbacks.save) (lock.clone(), push_history);
    }
    html!("input-form-textarea", {
        .property_signal("value", state.text_signal())
        .property("label", STR_INSTRUCTIONS_LABEL)
        .property("placeholder", STR_INSTRUCTIONS_PLACEHOLDER)
        .property("rows", 4)
        //Input saves every character
        //Change also pushes history
        .event(clone!(state => move |evt:events::CustomInput| {
            change_text(&state, evt.value(), false);
        }))
        .event(clone!(state => move |evt:events::CustomChange| {
            change_text(&state, evt.value(), true);
        }))
    })
}

pub fn render_audio(state: Rc<State>) -> Dom {

    let opts = AudioInputOptions {
        audio: state.instructions.get_cloned().audio
    };

    let callbacks = AudioCallbacks::new(
        Some(clone!(state => move |audio:Audio| {
            let mut lock = state.instructions.lock_mut();
            lock.audio = Some(audio); 
            (state.callbacks.save) (lock.clone(), true); 
        })),
        Some(clone!(state => move || {
            let mut lock = state.instructions.lock_mut();
            lock.audio = None; 
            (state.callbacks.save) (lock.clone(), true); 
        }))
    );

    let audio_state = Rc::new(AudioState::new(opts, callbacks)); 

    html!("empty-fragment", {
        .future(state.audio_signal()
                .to_stream()
                .skip(1)
                .for_each(clone!(audio_state => move |audio| {
                    //This just happens when history is changed really
                    audio_state.set_audio_ext(audio);
                    async {}
                }))
        )
        .child(render_audio_input(audio_state, None))
    })
}
