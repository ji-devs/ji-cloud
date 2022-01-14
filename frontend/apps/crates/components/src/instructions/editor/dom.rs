use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;
use utils::prelude::*;
use web_sys::HtmlTextAreaElement;

use super::state::State;
use crate::audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::Audio;

pub const STR_INSTRUCTIONS_LABEL: &str = "Written instructions";
pub const STR_INSTRUCTIONS_PLACEHOLDER: &str = "Type instructions";

pub fn render(state: Rc<State>) -> Dom {
    html!("div", {
        .children(&mut [
            render_text(state.clone()),
            render_audio(state)
        ])
    })
}

pub fn render_text(state: Rc<State>) -> Dom {
    fn change_text(state: &State, text: String, push_history: bool) {
        let mut lock = state.instructions.lock_mut();

        if text.is_empty() {
            lock.text = None;
        } else {
            lock.text = Some(text);
        }
        (state.callbacks.save)(lock.clone(), push_history);
    }
    html!("input-wrapper", {
        .property("label", STR_INSTRUCTIONS_LABEL)
        .child(html!("textarea" => HtmlTextAreaElement, {
            .with_node!(elem => {
                .text_signal(state.text_signal())
                .property("placeholder", STR_INSTRUCTIONS_PLACEHOLDER)
                .property("rows", 4)
                //Input saves every character
                //Change also pushes history
                .event(clone!(state => move |_:events::Input| {
                    change_text(&state, elem.value(), false);
                }))
                .event(clone!(state => move |evt:events::Change| {
                    let target = evt.dyn_target::<HtmlTextAreaElement>().unwrap_ji();
                    change_text(&state, target.value(), true);
                }))
            })
        }))
    })
}

pub fn render_audio(state: Rc<State>) -> Dom {
    let opts = AudioInputOptions::new(Some(
        state
            .instructions
            .signal_cloned()
            .map(|instructions| instructions.audio),
    ));

    let callbacks = AudioInputCallbacks::new(
        Some(clone!(state => move |audio:Audio| {
            let mut lock = state.instructions.lock_mut();
            lock.audio = Some(audio);
            (state.callbacks.save) (lock.clone(), true);
        })),
        Some(clone!(state => move || {
            let mut lock = state.instructions.lock_mut();
            lock.audio = None;
            (state.callbacks.save) (lock.clone(), true);
        })),
    );

    AudioInput::render(AudioInput::new(opts, callbacks), None)
}
