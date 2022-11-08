use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;
use utils::prelude::*;
use web_sys::HtmlTextAreaElement;

use super::state::State;
use crate::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    hebrew_buttons::HebrewButtons,
};
use futures_signals::{map_ref, signal::SignalExt};
use shared::domain::module::body::Audio;
use unicode_segmentation::UnicodeSegmentation;

const MAX_INSTRUCTION_TEXT_LEN: usize = 200;

pub fn render(state: Rc<State>) -> Dom {
    html!("div", {
        .children(&mut [
            render_text(state.clone()),
            render_audio(state)
        ])
    })
}

pub fn render_text(state: Rc<State>) -> Dom {
    fn change_text(state: &State, elem: &HtmlTextAreaElement, push_history: bool) {
        let text = elem.value();
        let limited_text = limit_text(MAX_INSTRUCTION_TEXT_LEN, text.clone());
        let mut lock = state.instructions.lock_mut();

        if limited_text.is_empty() {
            lock.text = None;
        } else {
            lock.text = Some(limited_text.clone());
        }
        (state.callbacks.save)(lock.clone(), push_history);

        if limited_text != text {
            elem.set_value(&limited_text);
        }
    }

    let char_limit_signal = map_ref! {
        let text = state.text_signal()
        => {
            let count = text.graphemes(true).count();
            format!("({}/{})", count, MAX_INSTRUCTION_TEXT_LEN)
        }
    };

    html!("input-wrapper", {
        .prop_signal("label", char_limit_signal.map(clone!(state => move |count_text| {
            format!("{} {}", state.instructions_text.label, count_text)
        })))
        .child(HebrewButtons::reveal().render(Some("hebrew-inputs")))
        .child(html!("textarea" => HtmlTextAreaElement, {
            .with_node!(elem => {
                .attr("dir", "auto")
                .text_signal(state.text_signal())
                .prop("placeholder", state.instructions_text.placeholder)
                .prop("rows", 4)
                //Input saves every character
                //Change also pushes history
                .event(clone!(state => move |_: events::Input| {
                    change_text(&state, &elem, false);
                }))
                .event(clone!(state => move |evt: events::Change| {
                    let target = evt.dyn_target::<HtmlTextAreaElement>().unwrap_ji();
                    change_text(&state, &target, true);
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
