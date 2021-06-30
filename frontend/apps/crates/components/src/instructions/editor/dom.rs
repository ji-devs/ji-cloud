use dominator::{html, Dom, clone};
use web_sys::HtmlSelectElement;
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
    html!("input-wrapper", {
        .property("label", STR_INSTRUCTIONS_LABEL)
        .child(html!("textarea", {
            .text_signal(state.text_signal())
            .property("placeholder", STR_INSTRUCTIONS_PLACEHOLDER)
            .property("rows", 4)
            //Input saves every character
            //Change also pushes history
            .event(clone!(state => move |evt:events::Input| {
                change_text(&state, evt.value().unwrap_or_default(), false);
            }))
            .event(clone!(state => move |evt:events::Change| {
                let target = evt.dyn_target::<HtmlSelectElement>().unwrap();
                change_text(&state, target.value(), true);
            }))
        }))
    })
}

pub fn render_audio(state: Rc<State>) -> Dom {
    let opts = AudioInputOptions::new(
        Some(state.instructions.signal_cloned().map(|instructions| instructions.audio))
    );

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

    let audio_state = AudioState::new(opts, callbacks); 

    render_audio_input(audio_state, None)
}
